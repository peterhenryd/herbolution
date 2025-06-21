//! This module defines the Herbolution application structure and system-level behavior with a render and update cycle.

use std::path::PathBuf;
use std::time::Duration;

use engine::{Engine, input, video};
use math::ext::{ext2u, Ext2};
use math::vec::Vec2;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::{DeviceEvent, DeviceId, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowId};
pub use crate::app::state::State;
pub use crate::app::store::Store;
pub use crate::app::switch::Switch;

pub mod store;
pub mod state;
pub mod switch;

/// An Herbolution application.
pub struct App<'w> {
    /// The data that persists through the entire duration of the program irrespective of the operating system or user directive.
    store: Store,
    /// The state for the active portion of the application as determined by the user.
    state: State,
    /// The suspended or resumed window and engine as determined by the operating system.
    switch: Switch<'w>,
}

/// Options for configuring an Herbolution application.
pub struct Options {
    /// The root path directory of the application. See [lib::fs::Fs::new] for more details.
    pub root_path: PathBuf,
}

/// A portable context frame that contains data for updating the application state.
pub struct Update<'w, 'a> {
    // Application data
    pub persist: &'a mut Store,
    pub window: &'a Window,
    pub engine: &'a mut Engine<'w>,
    pub event_loop: &'a ActiveEventLoop,

    // Frame data
    pub dt: Duration,
    pub input: input::Frame,
}

/// A portable context frame that contains data for rendering the application state.
pub struct Render<'a> {
    // Application data
    pub persist: &'a mut Store,

    // Frame data
    pub drawing: video::Frame<'a, 'a>,
    pub resolution: ext2u,
}

impl App<'_> {
    /// Creates a new Herbolution application with the specified options.
    pub fn new(options: Options) -> Self {
        Self {
            store: Store::new(options.root_path),
            state: State::default(),
            switch: Switch::Suspended(None),
        }
    }

    /// Runs the Herbolution application with `winit`.
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        EventLoop::new()?.run_app(self)
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // If the application was suspended, initialize or resume the window and create a new engine.
        self.switch.resume(event_loop);
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
                    .set_resolution(Ext2::new(width, height));
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                self.state = State::Exiting;
                update(&mut self.store, &mut self.state, window, engine, event_loop);
            }
            WindowEvent::CursorMoved {
                position: PhysicalPosition { x, y },
                ..
            } => {
                engine
                    .input
                    .set_mouse_pos(Vec2::new(x, y));
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
                engine
                    .input
                    .add_mouse_scroll(delta.y as f32);
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
            }
            WindowEvent::RedrawRequested => {
                // Update and render the application.
                update(&mut self.store, &mut self.state, window, engine, event_loop);
                render(&mut self.store, &mut self.state, engine);

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

fn update(persist: &mut Store, state: &mut State, window: &Window, engine: &mut Engine, event_loop: &ActiveEventLoop) {
    // Get the duration of time passed and user inputs that occurred between now and the previous update.
    let dt = persist.delta_time.next();
    let input = engine.input.take_frame();

    persist.fps.update(dt);

    // Update the state of the application.
    state.update(&mut Update {
        persist,
        window,
        engine,
        event_loop,
        dt,
        input,
    });
}

fn render(persist: &mut Store, state: &mut State, engine: &mut Engine) {
    // Render the state of the application to the window's surface.
    let resolution = engine.video.resolution();
    let drawing = engine.video.start_drawing();
    state.render(&mut Render { persist, drawing, resolution });

    /* Frame is automatically submitted to the GPU for rendering. */
}
