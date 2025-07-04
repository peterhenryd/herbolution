use lib::color::Rgba;
use lib::rotation::Quat;
use lib::vector::{Vec2, vec2f};
use wgpu::BufferUsages;

use crate::video::resource::GrowBuffer;
use crate::video::ui::atlas::Atlas;
use crate::video::ui::brush::Brush;
use crate::video::ui::font::FontId;
use crate::video::ui::vertex::Instance2d;

pub struct TextBrush<'h, 'f, 'a, 'b> {
    brush: &'b mut Brush<'h, 'f, 'a>,
    pub atlas: &'b Atlas,
    instances: Vec<Instance2d>,
    instance_buffer: GrowBuffer<Instance2d>,
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
        let instance_buffer = GrowBuffer::empty(brush.frame.handle, BufferUsages::VERTEX | BufferUsages::COPY_DST);

        Self {
            brush,
            atlas,
            instances: vec![],
            instance_buffer,
        }
    }

    pub fn add(&mut self, position: vec2f, text: &Text) {
        let mut x = 0.0;

        for char in text.content.chars() {
            let coord = self
                .atlas
                .glyph_coord(text.font_id, char, text.font_size)
                .unwrap();

            let position = vec2f::new(position.x + x, position.y);

            self.instances.push(Instance2d::new(
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
