//! This module defines the Herbolution application structure and system-level behavior with a video and update cycle.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use lib::color::{Color, ColorConsts, Rgba};
use lib::fs::Fs;
use lib::save::Save;
use lib::size::{Size2, size2u};
use lib::util::DeltaTime;
use lib::vector::Vec2;
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
use crate::video::Video;
use crate::video::resource::SampleCount;
use crate::video::ui::brush::Text;

pub struct App<'w> {
    store: Store,

    state: State,

    switch: Switch<'w>,
    init: bool,
}

pub struct Update<'w, 'a> {
    pub store: &'a mut Store,
    pub window: &'a Window,
    pub event_loop: &'a ActiveEventLoop,
    pub video: &'a mut Video<'w>,

    pub dt: Duration,
    pub input: InputFrame,
}

pub struct Render<'a> {
    pub store: &'a mut Store,

    pub frame: video::Frame<'a, 'a>,
    pub resolution: size2u,
}

impl App<'_> {
    pub fn new(root_dir: Option<PathBuf>) -> Self {
        Self {
            store: Store::new(root_dir),
            state: State::default(),
            switch: Switch::Suspended(None),
            init: false,
        }
    }

    pub fn run(&mut self) -> Result<(), EventLoopError> {
        EventLoop::new()?.run_app(self)
    }

    fn init(&mut self) {
        self.store
            .fs
            .init()
            .expect("Failed to init Herbolution file system");
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.init {
            self.init();
            self.init = true;
        }

        self.switch
            .resume(event_loop, self.store.fs.path().join("assets"));
    }

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

pub struct Store {
    pub(crate) input: Input,
    pub(crate) fs: Fs,
    pub(crate) delta_time: DeltaTime,
}

impl Store {
    pub fn new(root_dir: Option<PathBuf>) -> Self {
        Self {
            input: Input::default(),
            fs: Fs::new(root_dir),
            delta_time: DeltaTime::new(),
        }
    }
}

pub enum Switch<'w> {
    Resumed { window: Arc<Window>, video: Video<'w> },
    Suspended(Option<Arc<Window>>),
}

const RESOLUTION: (u32, u32) = (1920, 1080);

impl Switch<'_> {
    pub(super) fn resume(&mut self, event_loop: &ActiveEventLoop, asset_path: PathBuf) {
        let cached_window;
        match self {
            Switch::Resumed { .. } => return,
            Switch::Suspended(window) => cached_window = window.take(),
        }

        let window = cached_window.unwrap_or_else(|| create_window(event_loop));
        let video = Video::create(
            window.clone(),
            video::Options {
                resolution: RESOLUTION.into(),
                clear_color: Rgba::<u8>::from_rgb(117, 255, 250).into(),
                sample_count: SampleCount::Multi,
                asset_path,
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

fn create_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const RESOLUTION: (u32, u32) = (1920, 1080);

    let attributes = WindowAttributes::default()
        .with_title(format!("Herbolution {}", VERSION))
        .with_inner_size::<PhysicalSize<u32>>(RESOLUTION.into());
    let window = event_loop
        .create_window(attributes)
        .expect("Failed to create window");

    Arc::new(window)
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

    pub fn render(&mut self, ctx: &mut Render) {
        match self {
            State::Loading(splash) => splash.render(ctx),
            State::Browsing(menu) => menu.render(ctx),
            State::Playing(session) => session.render(ctx),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Loading(Splash::default())
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
    pub fn update(&mut self, ctx: &mut Update) -> Option<Command> {
        self.lifetime = self.lifetime.saturating_sub(ctx.dt);

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

impl Default for Splash {
    fn default() -> Self {
        Self {
            lifetime: Duration::from_millis(750),
        }
    }
}
