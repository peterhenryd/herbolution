use engine::painter::text::{Text, TextBrush};
use math::color::{ColorConsts, Rgba};
use math::vec::{vec3d, Vec2};
use winit::keyboard::KeyCode;

use crate::app::Update;

#[derive(Debug, Default)]
pub struct Debugger {
    is_enabled: bool,
}

impl Debugger {
    pub fn update(&mut self, ctx: &mut Update) {
        if ctx.input.key_events.contains(&KeyCode::Backquote) {
            self.is_enabled = !self.is_enabled;
        }
    }

    pub fn render(&mut self, fps: u64, player_position: vec3d, brush: &mut TextBrush) {
        if !self.is_enabled {
            return;
        }

        let font_id = brush.font_id();
        brush.add(
            Vec2::ZERO,
            &Text {
                font_id,
                content: format!("FPS: {fps}"),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.add(
            Vec2::new(0., 40.),
            &Text {
                font_id,
                content: format!("X: {:.2}", player_position.x),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.add(
            Vec2::new(0., 80.),
            &Text {
                font_id,
                content: format!("Y: {:.2}", player_position.y),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.add(
            Vec2::new(0., 120.),
            &Text {
                font_id,
                content: format!("Z: {:.2}", player_position.z),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );
    }
}
