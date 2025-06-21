use gpu::buffer::Usage;
use gpu::GrowBuffer;
use math::color::Rgba;
use math::rotation::Quat;
use math::vec::{vec2f, Vec2};

use crate::atlas::Atlas;
use crate::font::FontId;
use crate::vertex::{Instance2d, Instance2dPayload};
use crate::Brush;

pub struct TextBrush<'h, 'f, 'a, 'b> {
    brush: &'b mut Brush<'h, 'f, 'a>,
    pub atlas: &'b Atlas,
    instances: Vec<Instance2dPayload>,
    instance_buffer: GrowBuffer<Instance2dPayload>,
}

pub struct Text {
    pub font_id: FontId,
    pub content: String,
    pub font_size: f32,
    pub color: Rgba<f32>,
}

impl<'h, 'f, 'a, 'b> TextBrush<'h, 'f, 'a, 'b> {
    pub fn new(brush: &'b mut Brush<'h, 'f, 'a>, atlas: &'b Atlas) -> Self {
        brush.load_mesh(brush.painter.quad_mesh);
        let instance_buffer = GrowBuffer::empty(brush.frame.handle, Usage::VERTEX | Usage::COPY_DST);

        Self {
            brush,
            atlas,
            instances: vec![],
            instance_buffer,
        }
    }

    pub fn add(&mut self, position: vec2f, text: Text) {
        let mut x = 0.0;

        for char in text.content.chars() {
            let coord = self
                .atlas
                .glyph_coord(text.font_id, char, text.font_size)
                .unwrap();

            let position = vec2f::new(position.x + x, position.y);

            self.instances.push(
                Instance2d {
                    position,
                    rotation: Quat::IDENTITY,
                    scale: Vec2::new(coord.metrics.width as f32, coord.metrics.height as f32),
                    color: Rgba::TRANSPARENT,
                    texture_coord: coord.texture,
                }
                .payload(),
            );

            x += coord.metrics.advance_width;
        }
    }
}

impl Drop for TextBrush<'_, '_, '_, '_> {
    fn drop(&mut self) {
        if self.instances.is_empty() {
            return;
        }

        self.instance_buffer
            .write(self.brush.frame.handle, &self.instances);
        self.brush.render(&self.instance_buffer);
    }
}
