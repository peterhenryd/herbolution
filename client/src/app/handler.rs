use crate::app::App;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceEvent, DeviceId, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::WindowId;
use math::size::Size2;
use math::vector::Vec2;

#[derive(Default)]
pub struct Handler {
    app: Option<App>,
}

impl ApplicationHandler for Handler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app = Some(App::new(event_loop));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(app) = &mut self.app else { return };

        match event {
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                app.set_size(Size2::new(width, height));
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
                app.exit();
            }
            WindowEvent::CursorMoved { position: PhysicalPosition { x, y }, .. } => {
                app.engine.input.set_mouse_position(Vec2::new(x, y));
            }
            WindowEvent::MouseInput { button, state, .. } => {
                app.engine.input.set_mouse_button_activity(button, state.is_pressed());
            }
            WindowEvent::MouseWheel { delta: MouseScrollDelta::PixelDelta(delta), .. } => {
                app.engine.input.add_mouse_scroll(delta.y as f32);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                app.engine.input.set_key_activity(code, state.is_pressed());
            }
            WindowEvent::RedrawRequested => {
                app.update();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let Some(app) = &mut self.app else { return };

        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                app.engine.input.add_mouse_movement(Vec2::new(dx, dy));
            }
            _ => {}
        }
    }
}
