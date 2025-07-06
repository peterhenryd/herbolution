use crate::video::frame::Frame;
use crate::video::resource::{AtlasTextureCoord, Buffer, MeshId, SetId};
use crate::video::ui::text::TextBrush;
use crate::video::ui::vertex::Instance2d;
use crate::video::ui::{Painter, RenderType};
use lib::color::Rgba;
use lib::rotation::Quat;
use lib::vector::vec2f;
use wgpu::{BufferUsages, RenderPass};

pub struct Brush<'h, 'f, 'a> {
    pub frame: &'f mut Frame<'h>,
    pub painter: &'a Painter,
    mesh_index_count: Option<u32>,

    pub(crate) quads: Encoding,
}

impl<'h, 'f, 'a> Brush<'h, 'f, 'a> {
    pub fn create(render_type: RenderType, frame: &'f mut Frame<'h>, renderer: &'a Painter) -> Self {
        renderer
            .pipeline_map
            .load_by_type(render_type, frame.pass());

        Self {
            quads: Encoding::new(),
            frame,
            painter: renderer,
            mesh_index_count: None,
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

    pub fn draw_text(&mut self) -> TextBrush<'h, 'f, 'a, '_> {
        TextBrush::new(self)
    }

    pub fn draw_rect(&mut self, position: vec2f, scale: vec2f, color: Rgba<f32>) {
        self.quads
            .instances
            .push(Instance2d::new(position, Quat::IDENTITY, scale, color, AtlasTextureCoord::NONE));
    }
}

impl Drop for Brush<'_, '_, '_> {
    fn drop(&mut self) {
        if !self.quads.instances.is_empty() {
            let buffer = Buffer::from_data(self.frame.handle, &self.quads.instances, BufferUsages::VERTEX | BufferUsages::COPY_DST);
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

pub struct Encoding {
    pub(crate) instances: Vec<Instance2d>,
}

impl Encoding {
    pub fn new() -> Self {
        Self { instances: Vec::new() }
    }

    pub fn add(&mut self, instance: Instance2d) {
        self.instances.push(instance);
    }
}
