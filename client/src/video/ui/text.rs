use lib::color::{ColorConsts, Rgba};
use lib::rotation::Quat;
use lib::vector::{vec2f, Vec2};

use crate::video::ui::brush::Brush;
use crate::video::ui::font::FontId;
use crate::video::ui::vertex::Instance2d;

pub struct TextBrush<'h, 'f, 'a, 'b> {
    brush: &'b mut Brush<'h, 'f, 'a>,
}

#[derive(Debug)]
pub struct Text {
    pub font_id: FontId,
    pub content: String,
    pub font_size: f32,
    pub color: Rgba<f32>,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            font_id: FontId::default(),
            content: String::new(),
            font_size: 12.0,
            color: Rgba::BLACK,
        }
    }
}

impl<'h, 'f, 'a, 'b> TextBrush<'h, 'f, 'a, 'b> {
    pub fn new(brush: &'b mut Brush<'h, 'f, 'a>) -> Self {
        brush.load_mesh(brush.painter.quad_mesh);

        Self { brush }
    }

    pub fn add(&mut self, position: vec2f, text: &Text) {
        let mut x = 0.0;

        for char in text.content.chars() {
            let coord = self
                .brush
                .painter
                .atlas
                .glyph_coord(text.font_id, char, text.font_size)
                .unwrap();

            let position = vec2f::new(position.x + x + coord.metrics.xmin as f32, position.y + coord.metrics.ymin as f32);

            self.brush.quads.instances.push(Instance2d::new(
                position,
                Quat::IDENTITY,
                Vec2::new(coord.metrics.width as f32, coord.metrics.height as f32),
                Rgba::TRANSPARENT,
                coord.texture,
            ));

            x += coord.metrics.advance_width;
        }
    }

    pub fn font_id(&self) -> FontId {
        self.brush
            .painter
            .atlas
            .font_coords
            .iter()
            .next()
            .unwrap()
            .0
            .font_id
    }
}
