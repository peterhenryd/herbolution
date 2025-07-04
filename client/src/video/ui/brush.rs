use wgpu::RenderPass;

use crate::video::frame::Frame;
use crate::video::resource::{Buffer, MeshId, SetId};
use crate::video::ui::text::TextBrush;
use crate::video::ui::vertex::Instance2d;
use crate::video::ui::{Painter, RenderType};

pub struct Brush<'h, 'f, 'a> {
    pub frame: &'f mut Frame<'h>,
    pub painter: &'a Painter,
    mesh_index_count: Option<u32>,
}

impl<'h, 'f, 'a> Brush<'h, 'f, 'a> {
    pub fn create(render_type: RenderType, frame: &'f mut Frame<'h>, renderer: &'a Painter) -> Self {
        renderer
            .pipeline_map
            .load_by_type(render_type, frame.pass());

        Self {
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
        TextBrush::new(self, &self.painter.atlas)
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
