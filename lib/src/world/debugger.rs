use crate::game::fps::Fps;
use crate::listener::{InputEvent, Listener};
use crate::ui::text::TextSection;
use crate::ui::Ui;
use crate::world::transform::Transform;
use math::color::Color4;
use math::vector::vec2;
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::keyboard::KeyCode;

pub struct Debugger {
    is_enabled: bool,
}

impl Debugger {
    pub fn create() -> Self {
        Self {
            is_enabled: false
        }
    }

    pub fn update(&mut self, ui: &mut Ui, fps: &Fps, transform: Transform) {
        // Makeshift cross-hair
        let PhysicalSize { width, height } = ui.get_text_renderer().get_resolution();
        ui.push_text(TextSection {
            position: vec2::new(width as f32 / 2.0 - 1.5, height as f32 / 2.0 - 1.5),
            content: "+".to_string(),
            font_size: 24.0,
            line_height: 24.0,
            color: Color4::WHITE,
        });

        if !self.is_enabled {
            return;
        }

        ui.push_text(TextSection {
            position: vec2::new(10., 10.),
            content: format!("FPS: {}", fps.get()),
            font_size: 36.0,
            line_height: 42.0,
            color: Color4::WHITE,
        });

        ui.push_text(TextSection {
            position: vec2::new(10., 46.),
            content: format!("X: {:.2}", transform.position.x),
            font_size: 36.0,
            line_height: 42.0,
            color: Color4::WHITE,
        });

        ui.push_text(TextSection {
            position: vec2::new(10., 82.),
            content: format!("Y: {:.2}", transform.position.y),
            font_size: 36.0,
            line_height: 42.0,
            color: Color4::WHITE,
        });

        ui.push_text(TextSection {
            position: vec2::new(10., 118.),
            content: format!("Z: {:.2}", transform.position.z),
            font_size: 36.0,
            line_height: 42.0,
            color: Color4::WHITE,
        });
    }
}

impl Listener for Debugger {
    fn on_window_resized(&mut self, _: PhysicalSize<u32>) {}

    fn on_input(&mut self, event: &InputEvent) {
        if let InputEvent::Key { code: KeyCode::Backslash, state: ElementState::Pressed } = event {
            self.is_enabled = !self.is_enabled;
        }
    }
}