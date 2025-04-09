use crate::app::App;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceEvent, DeviceId, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::WindowId;
use math::size::Size2;
use math::vector::Vec2;
use crate::Options;

pub struct Handler {
    options: Options,
    app: Option<App>,
}

impl Handler {
    pub fn new(options: Options) -> Self {
        Self { options, app: None }
    }
}

impl ApplicationHandler for Handler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app = Some(App::new(event_loop, &self.options.data_dir));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(app) = &mut self.app else { return };

        match event {
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                app.set_size(Size2::new(width, height));
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                app.exit();
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position: PhysicalPosition { x, y }, .. } => {
                app.engine.input.set_mouse_pos(Vec2::new(x, y));
            }
            WindowEvent::MouseInput { button, state, .. } => {
                app.engine.input.set_mouse_button_activity(button, state.is_pressed());
            }
            WindowEvent::MouseWheel { delta: MouseScrollDelta::PixelDelta(delta), .. } => {
                app.engine.input.add_mouse_scroll(delta.y as f32);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                app.engine.input.set_modifiers(modifiers);
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
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
