use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

pub trait Listener {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>);

    fn on_input(&mut self, event: &InputEvent);
}

pub enum InputEvent {
    Key { code: KeyCode, state: ElementState },
    MouseMoving { dx: f64, dy: f64 },
    MouseMoved(PhysicalPosition<f64>),
    MouseClick { button: MouseButton, state: ElementState },
    MouseWheel { delta: f32 },
    MouseEntered,
    MouseLeft,
}