//! This module defines the Herbolution application structure and system-level behavior with a video and update cycle.

use std::ffi::{OsStr, OsString};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use clap::builder::{BoolValueParser, TypedValueParser};
use clap::error::ErrorKind;
use clap::{Arg, Error, Parser};
use lib::color::{Color, ColorConsts, Rgba};
use lib::fs::Fs;
use lib::save::Save;
use lib::size::{size2u, Size2};
use lib::util::DeltaTime;
use lib::vector::Vec2;
use time::Duration;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::{DeviceEvent, DeviceId, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::input::{Input, InputFrame};
use crate::menu::{Menu, MenuConfig};
use crate::session::Session;
use crate::video;
use crate::video::resource::SampleCount;
use crate::video::ui::brush::Text;
use crate::video::Video;

#[derive(Debug)]
pub struct App<'w> {
    store: Store,
    state: State,
    switch: Switch<'w>,
    options: AppOptions,
}

impl App<'_> {
    pub fn new(options: AppOptions) -> Self {
        let store = Store::new(options.data_dir.clone());

        store
            .fs
            .init()
            .expect("Failed to init Herbolution file system");

        Self {
            store,
            state: State::Loading(Splash::new()),
            switch: Switch::Suspended(None),
            options,
        }
    }

    pub fn run(&mut self) -> Result<(), EventLoopError> {
        EventLoop::new()?.run_app(self)
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.switch
            .resume(event_loop, self.store.fs.path().join("assets"), &self.options);
    }

    #[tracing::instrument(skip(self))]
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Switch::Resumed { window, video } = &mut self.switch else {
            return;
        };

        match event {
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                video.set_resolution(Size2::new(width, height));
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                self.state.transition(
                    Command::Exit,
                    &mut Update {
                        store: &mut self.store,
                        window,
                        video,
                        event_loop,
                        dt: Duration::ZERO,
                        input: InputFrame::default(),
                    },
                );
            }
            WindowEvent::CursorMoved {
                position: PhysicalPosition { x, y },
                ..
            } => {
                self.store
                    .input
                    .set_mouse_position(Vec2::new(x, y));
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.store
                    .input
                    .set_mouse_button_activity(button, state.is_pressed());
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::PixelDelta(delta),
                ..
            } => {
                self.store.input.add_mouse_scroll(delta.y as f32);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.store.input.set_modifiers(modifiers);
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    ..
                },
                ..
            } => {
                self.store
                    .input
                    .push_key_activity(code, state.is_pressed());

                if code == KeyCode::Backslash {
                    video.set_sample_count(match video.surface.sample_count() {
                        SampleCount::Single => SampleCount::Multi,
                        SampleCount::Multi => SampleCount::Single,
                    });
                }
            }
            WindowEvent::RedrawRequested => {
                #[cfg(feature = "tracing")]
                tracing_tracy::client::frame_mark();

                let input = self.store.input.take_frame();
                self.state.update(&mut Update {
                    dt: self.store.delta_time.next(),
                    store: &mut self.store,
                    input,
                    window,
                    video,
                    event_loop,
                });

                self.state.render(&mut Render {
                    store: &mut self.store,
                    resolution: video.resolution(),
                    frame: video.create_frame(),
                });

                window.request_redraw();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let Switch::Resumed { .. } = &mut self.switch else {
            return;
        };

        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                self.store
                    .input
                    .push_mouse_movement(Vec2::new(dx, dy));
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.switch.suspend();
    }
}

#[derive(Debug)]
pub struct Store {
    pub(crate) input: Input,
    pub(crate) fs: Fs,
    pub(crate) delta_time: DeltaTime,
}

impl Store {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            input: Input::default(),
            fs: Fs::new(root_dir),
            delta_time: DeltaTime::new(),
        }
    }
}

#[derive(Debug)]
pub enum Switch<'w> {
    Resumed { window: Arc<Window>, video: Video<'w> },
    Suspended(Option<Arc<Window>>),
}

impl Switch<'_> {
    pub(super) fn resume(&mut self, event_loop: &ActiveEventLoop, asset_path: PathBuf, options: &AppOptions) {
        let cached_window;
        match self {
            Switch::Resumed { .. } => return,
            Switch::Suspended(window) => cached_window = window.take(),
        }

        let window = cached_window.unwrap_or_else(|| create_window(event_loop, options.resolution));
        let video = Video::create(
            window.clone(),
            video::Options {
                resolution: options.resolution,
                clear_color: Rgba::<u8>::from_rgb(117, 255, 250).into(),
                sample_count: options.sample_count,
                asset_path,
                vsync: options.vsync,
            },
        );

        *self = Switch::Resumed { window, video };
    }

    pub(super) fn suspend(&mut self) {
        let cached_window;
        if let Self::Resumed { window, .. } = self {
            cached_window = Some(Arc::clone(window));
        } else {
            cached_window = None;
        }

        *self = Switch::Suspended(cached_window);
    }
}

#[derive(Debug)]
pub struct Update<'w, 'a> {
    pub store: &'a mut Store,
    pub window: &'a Window,
    pub event_loop: &'a ActiveEventLoop,
    pub video: &'a mut Video<'w>,

    pub dt: Duration,
    pub input: InputFrame,
}

#[derive(Debug)]
pub struct Render<'a> {
    pub store: &'a mut Store,

    pub frame: video::Frame<'a, 'a>,
    pub resolution: size2u,
}

#[derive(Debug)]
pub enum State {
    Loading(Splash),
    Browsing(Menu),
    Playing(Session),
}

impl State {
    pub fn update(&mut self, context: &mut Update) {
        let command = match self {
            State::Loading(splash) => splash.update(context),
            State::Browsing(menu) => menu.update(context),
            State::Playing(session) => session.update(context),
        };

        if let Some(x) = command {
            self.transition(x, context);
        };
    }

    pub(crate) fn transition(&mut self, command: Command, ctx: &mut Update) {
        match command {
            Command::OpenMenu(config) => {
                *self = State::Browsing(Menu::new(config, &ctx.video.painter));
            }
            Command::StartGame { save } => {
                let session = Session::create(save, &mut ctx.video, &ctx.store.fs.path().join("assets"));
                *self = Self::Playing(session);
            }
            Command::Exit => {
                ctx.event_loop.exit();
            }
            Command::PauseGame => {
                if let State::Playing(session) = self {
                    session.pause()
                }
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn render(&mut self, ctx: &mut Render) {
        match self {
            State::Loading(splash) => splash.render(ctx),
            State::Browsing(menu) => menu.render(ctx),
            State::Playing(session) => session.render(ctx),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    OpenMenu(MenuConfig),
    StartGame { save: Save },
    PauseGame,
    Exit,
}

#[derive(Debug)]
pub struct Splash {
    lifetime: Duration,
}

impl Splash {
    pub fn new() -> Self {
        Self {
            lifetime: Duration::milliseconds(750),
        }
    }

    pub fn update(&mut self, ctx: &mut Update) -> Option<Command> {
        self.lifetime -= ctx.dt;

        if self.lifetime > Duration::ZERO {
            None
        } else {
            Some(Command::OpenMenu(MenuConfig::Title))
        }
    }

    pub fn render(&mut self, ctx: &mut Render) {
        ctx.frame
            .clear_color(Rgba::from_rgb(20u8, 40, 80).into());

        let mut brush = ctx.frame.draw_2d();

        let font_id = brush.default_font_id();
        brush.draw_text(
            (ctx.resolution.to_vec2().cast::<f32>() - Vec2::new(504.576, 56.0)) / 2.0,
            &Text {
                font_id,
                content: "Herbolution".to_string(),
                font_size: 96.0,
                color: Rgba::WHITE,
            },
        );
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "herbolution",
    about = "The client for Herbolution, a 3D voxel game.",
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct AppOptions {
    #[arg(
        default_value = "1920x1080",
        help = "Resolution of the window in physical pixels",
        long = "resolution",
        short = 'r',
        value_name = "WIDTHxHEIGHT",
        value_parser = Size2ValueParser::<u32>::new(),
    )]
    pub resolution: size2u,
    #[arg(
        default_value = default_root_dir().as_os_str(),
        help = "Directory for Herbolution data",
        long = "data-dir",
        short = 'd',
        value_name = "PATH",
        value_parser = clap::value_parser!(PathBuf),
    )]
    pub data_dir: PathBuf,
    #[arg(
        default_value = num_cpus_os_str(),
        help = "Number of CPU cores utilized for background tasks (overrides `HERBOLUTION_WORKER_THREADS`)",
        long = "workers",
        short = 'w',
        value_name = "NUM_THREADS",
    )]
    pub workers: usize,
    #[arg(
        action = clap::ArgAction::SetTrue,
        default_value = "false",
        help = "Enable multisample anti-aliasing",
        long = "msaa",
        short = 'a',
        value_parser = SampleCountValueParser,
    )]
    pub sample_count: SampleCount,
    #[arg(default_value = "false", help = "Limit frame rate to display refresh rate", long = "vsync", short = 'v')]
    pub vsync: bool,
}

#[derive(Debug)]
struct Size2ValueParser<T>(PhantomData<T>);

impl<T> Size2ValueParser<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for Size2ValueParser<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> Copy for Size2ValueParser<T> {}

impl<T: FromStr + Clone + Send + Sync + 'static> TypedValueParser for Size2ValueParser<T> {
    type Value = Size2<T>;

    fn parse_ref(&self, _: &clap::Command, _: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
        let value_string = value.to_string_lossy();
        let [width, height] = value_string
            .split('x')
            .next_chunk::<2>()
            .map_err(|_| Error::new(ErrorKind::InvalidValue))?;

        Ok(Size2 {
            width: T::from_str(width).map_err(|_| Error::new(ErrorKind::InvalidValue))?,
            height: T::from_str(height).map_err(|_| Error::new(ErrorKind::InvalidValue))?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct SampleCountValueParser;

impl TypedValueParser for SampleCountValueParser {
    type Value = SampleCount;

    fn parse_ref(&self, cmd: &clap::Command, arg: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
        Ok(BoolValueParser::new()
            .parse_ref(cmd, arg, value)?
            .then_some(SampleCount::Multi)
            .unwrap_or(SampleCount::Single))
    }
}

fn create_window(event_loop: &ActiveEventLoop, resolution: size2u) -> Arc<Window> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let attributes = WindowAttributes::default()
        .with_title(format!("Herbolution {VERSION}"))
        .with_inner_size::<PhysicalSize<u32>>(resolution.to_tuple().into());
    let window = event_loop
        .create_window(attributes)
        .expect("Failed to create window");

    Arc::new(window)
}

fn default_root_dir() -> &'static Path {
    directories::BaseDirs::new()
        .map(|dirs| dirs.data_local_dir().join("Herbolution"))
        .unwrap_or_else(|| {
            tracing::error!("Failed to find local data directory; using '.herbolution' in the working directory as the root directory.");
            PathBuf::from(".herbolution")
        })
        .leak()
}

fn num_cpus_os_str() -> &'static OsStr {
    OsString::from(num_cpus::get().to_string()).leak()
}
