//! This module defines the Herbolution application structure and system-level behavior with a render and update cycle.

use std::path::PathBuf;
use std::time::Duration;

pub use crate::app::state::State;
pub use crate::app::store::Store;
pub use crate::app::switch::Switch;
use engine::input::InputFrame;
use engine::{video, Engine};
use gpu::texture::SampleCount;
use math::size::{size2u, Size2};
use math::vec::Vec2;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::{DeviceEvent, DeviceId, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub mod state;
pub mod store;
pub mod switch;

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
    /// The root directory path of the application. See [lib::fs::Fs::new] for more details.
    pub root_dir: Option<PathBuf>,
}

/// A portable context frame that contains data for updating the application state.
pub struct Update<'w, 'a> {
    // Application data
    pub store: &'a mut Store,
    pub window: &'a Window,
    pub engine: &'a mut Engine<'w>,
    pub event_loop: &'a ActiveEventLoop,

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
        self.switch.resume(event_loop);

        if !self.init {
            self.init();
            self.init = true;
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // If the application was suspended by the operating system, do not handle window events.
        let Switch::Resumed { window, engine } = &mut self.switch else {
            return;
        };

        // Handle the window event and update the application accordingly.
        match event {
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                engine
                    .video
                    .set_resolution(Size2::new(width, height));
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                self.state = State::Exiting;
                self.state.update(&mut Update {
                    store: &mut self.store,
                    window,
                    engine,
                    event_loop,
                    dt: Duration::ZERO,
                    input: InputFrame::default(),
                });
            }
            WindowEvent::CursorMoved {
                position: PhysicalPosition { x, y },
                ..
            } => {
                engine.input.set_mouse_position(Vec2::new(x, y));
            }
            WindowEvent::MouseInput { button, state, .. } => {
                engine
                    .input
                    .set_mouse_button_activity(button, state.is_pressed());
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::PixelDelta(delta),
                ..
            } => {
                engine.input.add_mouse_scroll(delta.y as f32);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                engine.input.set_modifiers(modifiers);
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    ..
                },
                ..
            } => {
                engine
                    .input
                    .push_key_activity(code, state.is_pressed());

                if code == KeyCode::Backslash {
                    engine
                        .video
                        .set_sample_count(match engine.video.surface.sample_count() {
                            SampleCount::Single => SampleCount::Multi,
                            SampleCount::Multi => SampleCount::Single,
                        });
                }
            }
            WindowEvent::RedrawRequested => {
                // Update the application.
                self.state.update(&mut Update {
                    dt: self.store.delta_time.next(),
                    store: &mut self.store,
                    input: engine.input.take_frame(),
                    window,
                    engine,
                    event_loop,
                });

                // Render the application.
                self.state.render(&mut Render {
                    store: &mut self.store,
                    resolution: engine.video.resolution(),
                    frame: engine.video.create_frame(),
                });

                // Request a redraw from the operating system to repeat the update and render cycle.
                window.request_redraw();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let Switch::Resumed { engine, .. } = &mut self.switch else {
            return;
        };

        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                engine
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
