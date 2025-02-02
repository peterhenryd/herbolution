use std::sync::Arc;
use winit::event::{DeviceEvent, WindowEvent};
use winit::window::Window;
use herbolution_engine::Engine;
use crate::runtime::async_app::AsyncApp;

pub struct App {
    engine: Engine<'static>,
    window: Arc<Window>,
}

impl AsyncApp for App {
    async fn create(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self {
            engine: Engine::create(window.clone()).await?,
            window,
        })
    }

    async fn on_window_event(&mut self, event: WindowEvent) -> anyhow::Result<()> {
        use WindowEvent::*;
        match event {
            Resized(size) => {
                self.engine.resize(size);
                self.engine.surface.resize(size);
            }
            CursorMoved { position, .. } => {
                self.engine.window.cursor_position = position;
            }
            CloseRequested => {
                self.state.exit();
                std::process::exit(0);
            }
            KeyboardInput {
                event:
                KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(key),
                    ..
                },
                ..
            } => {
                self.engine
                    .input
                    .update(InputEvent::Type { key, state }, &mut self.state);
            }
            MouseInput { state, button, .. } => {
                self.engine.input.update(
                    InputEvent::Click {
                        button,
                        state,
                        x: self.engine.window.cursor_position.x,
                        y: self.engine.window.cursor_position.y,
                    },
                    &mut self.state,
                );
            }
            RedrawRequested => {
                self.state.update(&mut self.engine);
                self.state.render(&mut self.engine);

                self.engine.window.request_redraw();
            }
            _ => {}
        }
    }

    async fn on_device_event(&mut self, event: DeviceEvent) -> anyhow::Result<()> {
        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                self.engine
                    .input
                    .update(InputEvent::Move { dx, dy }, &mut self.state);
            }
            _ => {}
        }
    }
}