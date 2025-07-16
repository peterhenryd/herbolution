use lib::aabb::Aabb2;
use lib::color::{ColorConsts, Rgba};
use lib::rotation::Quat;
use lib::size::Size2;
use lib::vector::vec2f;
use wgpu::{BufferUsages, RenderPass};

use crate::video::frame::Frame;
use crate::video::resource::{AtlasTextureCoord, Buffer, MeshId, SetId};
use crate::video::ui::font::FontId;
use crate::video::ui::vertex::Instance2d;
use crate::video::ui::{Painter, RenderType};

pub struct Brush<'f, 'a> {
    pub frame: &'f mut Frame<'a>,
    pub painter: &'a Painter,
    mesh_index_count: Option<u32>,
    quads: Vec<Instance2d>,
}

impl<'f, 'a> Brush<'f, 'a> {
    pub fn create(render_type: RenderType, frame: &'f mut Frame<'a>, painter: &'a Painter) -> Self {
        painter
            .pipeline_map
            .load_by_type(render_type, frame.pass());

        let mesh = painter.meshes.get(painter.quad_mesh);
        let mesh_index_count = Some(mesh.load_into_render_pass(&mut frame.pass()));

        Self {
            quads: vec![],
            frame,
            painter,
            mesh_index_count,
        }
    }

    pub fn load_mesh(&mut self, id: MeshId) {
        let mesh = self.painter.meshes.get(id);
        self.mesh_index_count = Some(mesh.load_into_render_pass(&mut self.frame.pass()));
    }

    pub fn render(&mut self, buffer: impl AsRef<Buffer<Instance2d>>) {
        draw_mesh(self.frame.pass(), buffer.as_ref(), self.mesh_index_count);
    }

    pub fn render_by_id(&mut self, id: SetId) {
        self.render(self.painter.instance_sets.get(id));
    }

    pub fn draw_rect(&mut self, bounds: Aabb2<f32>, color: Rgba<f32>, border_radius: f32) {
        self.quads.push(Instance2d::new(
            bounds.min,
            Quat::IDENTITY,
            bounds.size(),
            color,
            AtlasTextureCoord::NONE,
            border_radius,
        ));
    }

    #[tracing::instrument(skip_all)]
    pub fn draw_text(&mut self, position: vec2f, text: &Text) {
        let mut x = 0.0;

        for char in text.content.chars() {
            let coord = self
                .painter
                .atlas
                .glyph_coord(text.font_id, char, text.font_size)
                .unwrap();

            let position = vec2f::new(position.x + x + coord.metrics.xmin as f32, position.y + coord.metrics.ymin as f32);

            self.quads.push(Instance2d::new(
                position,
                Quat::IDENTITY,
                Size2::new(coord.metrics.width as f32, coord.metrics.height as f32),
                Rgba::new(text.color.r, text.color.g, text.color.b, 0.0),
                coord.texture,
                0.0,
            ));

            x += coord.metrics.advance_width;
        }
    }

    pub fn default_font_id(&self) -> FontId {
        self.painter.default_font_id()
    }
}

impl Drop for Brush<'_, '_> {
    fn drop(&mut self) {
        if !self.quads.is_empty() {
            let buffer = Buffer::from_data(self.frame.handle, &self.quads, BufferUsages::VERTEX | BufferUsages::COPY_DST);
            self.render(&buffer);
        }
    }
}

fn draw_mesh(pass: &mut RenderPass<'_>, buffer: &Buffer<Instance2d>, mesh_index_count: Option<u32>) {
    let Some(index_count) = mesh_index_count else {
        panic!("2D mesh must be loaded before drawing");
    };

    let buffer = buffer.as_ref();
    if buffer.len() == 0 {
        return;
    }

    pass.set_vertex_buffer(1, buffer.inner().slice(..));
    pass.draw_indexed(0..index_count, 0, 0..buffer.len() as u32);
}

#[derive(Debug, Clone)]
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
