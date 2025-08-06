use prims::size::Size2;
use prims::vector::Vec2;
use time::Duration;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceEvent, DeviceId, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::WindowId;

use crate::app::input::InputFrame;
use crate::app::state::Command;
use crate::app::{App, AppOptions, Render, Update};

#[derive(Debug)]
pub struct AppHandler {
    app: Option<App>,
    options: Option<AppOptions>,
}

impl AppHandler {
    pub fn new(options: AppOptions) -> Self {
        Self {
            app: None,
            options: Some(options),
        }
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let options = self.options.take().unwrap();
        self.app = Some(App::new(event_loop, options));
    }

    #[tracing::instrument(skip(self))]
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(app) = &mut self.app else {
            return;
        };

        match event {
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                app.video
                    .set_size(&app.gpu, Size2::new(width, height));
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                let (state, stateless_app_view) = app.split_state_borrow();
                state.transition(
                    Command::Exit,
                    &mut Update {
                        app: stateless_app_view,
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
                app.input.set_mouse_position(Vec2::new(x, y));
            }
            WindowEvent::MouseInput { button, state, .. } => {
                app.input
                    .set_mouse_button_activity(button, state.is_pressed());
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::PixelDelta(delta),
                ..
            } => {
                app.input.add_mouse_scroll(delta.y as f32);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                app.input.set_modifiers(modifiers);
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    ..
                },
                ..
            } => {
                app.input
                    .push_key_activity(code, state.is_pressed());
            }
            WindowEvent::RedrawRequested => {
                #[cfg(feature = "tracing")]
                tracing_tracy::client::frame_mark();

                let dt = app.delta_time.next();
                app.fps.update(dt);

                let input = app.input.take_frame();
                let (state, view) = app.split_state_borrow();

                let mut update = Update {
                    event_loop,
                    dt,
                    input,
                    app: view,
                };
                state.update(&mut update);
                let Update { app: view, .. } = update;

                let mut render = Render {
                    frame: view.video.create_frame(&view.gpu),
                    resolution: view.video.size(),
                    app: view,
                };
                state.render(&mut render);
                let Render { frame, .. } = render;

                frame.submit();
                app.window.request_redraw();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let Some(app) = &mut self.app else {
            return;
        };

        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                app.input.push_mouse_movement(Vec2::new(dx, dy));
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.app = None;
    }
}
