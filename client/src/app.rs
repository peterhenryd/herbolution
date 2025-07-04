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
use crate::menu::Menu;
use crate::menu::config::MenuConfig;
use crate::session::Session;
use crate::video;
use crate::video::resource::SampleCount;
use crate::video::ui::text::Text;
use crate::video::{Video, ui, world};

/// An Herbolution application.
pub struct App<'w> {
    /// The data that persists through the entire duration of the program irrespective of the operating system or user directive.
    store: Store,
    /// The state for the active portion of the application as determined by the user.
    state: State,
    /// The suspended or resumed window and herbolution_engine as determined by the operating system.
    switch: Switch<'w>,
    init: bool,
}

/// Options for configuring an Herbolution application.
pub struct Options {
    /// The root directory path of the application. See [herbolution_lib::world::fs::Fs::new] for more details.
    pub root_dir: Option<PathBuf>,
}

/// A portable context frame that contains data for updating the application state.
pub struct Update<'w, 'a> {
    // Application data
    pub store: &'a mut Store,
    pub window: &'a Window,
    pub event_loop: &'a ActiveEventLoop,
    pub video: &'a mut Video<'w>,

    // Frame data
    pub dt: Duration,
    pub input: InputFrame,
}

/// A portable context frame that contains data for rendering the application state.
pub struct Render<'a> {
    // Application data
    pub store: &'a mut Store,

    // Frame data
    pub frame: video::Frame<'a, 'a>,
    pub resolution: size2u,
}

impl App<'_> {
    /// Creates a new Herbolution application with the specified options.
    pub fn new(options: Options) -> Self {
        Self {
            store: Store::new(options.root_dir),
            state: State::default(),
            switch: Switch::Suspended(None),
            init: false,
        }
    }

    /// Runs the Herbolution application with `winit`.
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
        // If the application was suspended, initialize or resume the window and create a new herbolution_engine.
        self.switch
            .resume(event_loop, self.store.fs.path().to_path_buf());

        if !self.init {
            self.init();
            self.init = true;
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // If the application was suspended by the operating system, do not handle window events.
        let Switch::Resumed { window, video } = &mut self.switch else {
            return;
        };

        // Handle the window event and update the application accordingly.
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
                // Update the application.
                let input = self.store.input.take_frame();
                self.state.update(&mut Update {
                    dt: self.store.delta_time.next(),
                    store: &mut self.store,
                    input,
                    window,
                    video,
                    event_loop,
                });

                // Render the application.
                self.state.render(&mut Render {
                    store: &mut self.store,
                    resolution: video.resolution(),
                    frame: video.create_frame(),
                });

                // Request a redraw from the operating system to repeat the update and video cycle.
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

/// The data that persists through the entire duration of the application execution irrespective of the operating system or user directive.
///
/// This structure should only export data that is not dependent on the window, herbolution_engine, or navigation state.
pub struct Store {
    pub(crate) input: Input,
    pub(crate) fs: Fs,
    pub(crate) delta_time: DeltaTime,
}

impl Store {
    /// Creates a new instance with the specified file system root path.
    pub fn new(root_dir: Option<PathBuf>) -> Self {
        Self {
            input: Input::default(),
            fs: Fs::new(root_dir),
            delta_time: DeltaTime::new(),
        }
    }
}

/// A switch that allows the application to be resumed or suspended by the operating system gracefully.
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
                painter: ui::Options { texture_paths: vec![] },
                sculptor: world::Options {},
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

/// The navigable state of the application. This structure uses the senses provided by the herbolution_engine and persistent data during the update phase to
/// mutate itself for the following cycles.
#[derive(Debug)]
pub enum State {
    /// The initial state of the application, where it is loading resources. A splash screen is rendering during this state. Upon completion, it transitions to
    /// the title menu.
    Loading(Splash),
    /// The state where the user is not actively playing, and is viewing and interacting with a given menu.
    Browsing(Menu),
    /// The state where the user is playing the server. It also manages a potential overlay menu and has pause mechanics.
    Playing(Session),
}

impl State {
    /// Updates the current state of the application using the provided context.
    pub fn update(&mut self, context: &mut Update) {
        // Run the update behavior for the current state.
        let command = match self {
            State::Loading(splash) => splash.update(context),
            State::Browsing(menu) => menu.update(context),
            State::Playing(session) => session.update(context),
        };

        // If a command was returned, transition to its associated state.
        if let Some(x) = command {
            self.transition(x, context);
        };
    }

    // Replaces or mutates the current state based on the command.
    pub(crate) fn transition(&mut self, command: Command, ctx: &mut Update) {
        match command {
            Command::OpenMenu(config) => {
                *self = State::Browsing(config.into());
            }
            Command::StartGame { save } => {
                let session = Session::create(save, &mut ctx.video, ctx.store.fs.path());
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

    /// Renders the current state of the application using the provided context.
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

/// A state configuration used to construct the next state of the application.
#[derive(Debug, Clone)]
pub enum Command {
    /// Opens the specified menu.
    OpenMenu(MenuConfig),
    /// Starts the server with the specified save.
    StartGame {
        save: Save,
    },
    PauseGame,
    /// Exits the application.
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
        let mut text_brush = brush.draw_text();

        let font_id = text_brush.font_id();
        text_brush.add(
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
