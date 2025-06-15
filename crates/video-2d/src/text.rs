use gpu::buffer::Usage;
use gpu::{GrowBuffer, Handle};
use math::color::Rgba;
use math::rotation::Quat;
use math::vector::{Vec2, vec2f};

use crate::Drawing;
use crate::atlas::Atlas;
use crate::font::FontId;
use crate::vertex::{Instance2d, Instance2dPayload};

pub struct DrawText<'q, 'f, 'r, 'd, 'g> {
    handle: &'q Handle,
    drawing: &'d mut Drawing<'q, 'f, 'r>,
    pub atlas: &'g Atlas,
    instances: Vec<Instance2dPayload>,
    instance_buffer: GrowBuffer<Instance2dPayload>,
}

pub struct Text {
    pub font_id: FontId,
    pub content: String,
    pub font_size: f32,
    pub color: Rgba<f32>,
}

impl<'q, 'f, 'r, 'd, 'g> DrawText<'q, 'f, 'r, 'd, 'g> {
    pub fn new(handle: &'q Handle, drawing: &'d mut Drawing<'q, 'f, 'r>, atlas: &'g Atlas) -> Self {
        drawing.load_mesh(drawing.renderer.quad_mesh);

        Self {
            handle,
            drawing,
            atlas,
            instances: vec![],
            instance_buffer: GrowBuffer::empty(handle, Usage::VERTEX | Usage::COPY_DST),
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

impl Drop for DrawText<'_, '_, '_, '_, '_> {
    fn drop(&mut self) {
        if self.instances.is_empty() {
            return;
        }

        self.instance_buffer
            .write(self.handle, &self.instances);
        self.drawing.draw(&self.instance_buffer);
    }
}
