use lib::fps::IntervalCounter;
use math::color::{ColorConsts, Rgba};
use math::size::Size2;
use math::vector::{vec3d, Vec2};
use winit::keyboard::KeyCode;
use engine::input::InputFrame;
use engine::video::text::{TextFrame, TextId, TextSection};

pub struct Debugger {
    is_resized: bool,
    is_enabled: bool,
    size: Size2<u32>,
    crosshair_id: Option<TextId>,
    text_ids: Option<[TextId; 4]>,
}

impl Debugger {
    pub fn create(size: Size2<u32>) -> Self {
        Self { is_resized: true, is_enabled: false, size, crosshair_id: None, text_ids: None }
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.size = size;
        self.is_resized = true;
    }

    pub fn update(&mut self, input: &InputFrame, text: &mut TextFrame, fps: &IntervalCounter, position: vec3d) {
        if self.is_resized {
            if let Some(id) = self.crosshair_id.take() {
                text.remove(id);
            }

            self.crosshair_id = Some(text.insert(TextSection {
                pos: Vec2::new(self.size.width as f32 / 2.0, self.size.height as f32 / 2.0),
                content: "+".to_string(),
                font_size: 24.0,
                line_height: 24.0,
                color: Rgba::WHITE,
            }));
            self.is_resized = false;
        }

        if input.key_events.contains(&KeyCode::Backslash) {
            self.is_enabled = !self.is_enabled;
        }

        if let Some(ids) = self.text_ids.take() {
            for id in ids {
                text.remove(id);
            }
        }

        if !self.is_enabled {
            return;
        }

        let fps = text.insert(TextSection {
            pos: Vec2::new(10., 10.),
            content: format!("FPS: {}", fps.get()),
            font_size: 36.0,
            line_height: 42.0,
            color: Rgba::WHITE,
        });

        let x = text.insert(TextSection {
            pos: Vec2::new(10., 46.),
            content: format!("X: {:.2}", position.x),
            font_size: 36.0,
            line_height: 42.0,
            color: Rgba::WHITE,
        });

        let y = text.insert(TextSection {
            pos: Vec2::new(10., 82.),
            content: format!("Y: {:.2}", position.y),
            font_size: 36.0,
            line_height: 42.0,
            color: Rgba::WHITE,
        });

        let z = text.insert(TextSection {
            pos: Vec2::new(10., 118.),
            content: format!("Z: {:.2}", position.z),
            font_size: 36.0,
            line_height: 42.0,
            color: Rgba::WHITE,
        });

        self.text_ids = Some([fps, x, y, z]);
    }
}
