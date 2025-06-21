use gpu::frame::{Frame, Pass};
use gpu::{Buffer, MeshId, SetId};

use crate::vertex::Instance3dPayload;
use crate::{RenderType, Sculptor};

pub struct Chisel<'q, 'f, 'r> {
    frame: &'f mut Frame<'q>,
    renderer: &'r Sculptor,
    mesh_index_count: Option<u32>,
}

impl<'q, 'f, 'r> Chisel<'q, 'f, 'r> {
    pub fn create(render_type: RenderType, frame: &'f mut Frame<'q>, renderer: &'r Sculptor) -> Self {
        renderer
            .pipeline_map
            .load_by_type(render_type, frame.pass());

        Self {
            frame,
            renderer,
            mesh_index_count: None,
        }
    }

    pub fn load_mesh(&mut self, id: MeshId) {
        let mesh = self.renderer.meshes.get(id);
        self.mesh_index_count = Some(mesh.load_into_render_pass(&mut self.frame.pass()));
    }

    pub fn draw(&mut self, buffer: impl AsRef<Buffer<Instance3dPayload>>) {
        draw_mesh(self.frame.pass(), buffer.as_ref(), self.mesh_index_count);
    }

    pub fn draw_from_set(&mut self, id: SetId) {
        self.draw(self.renderer.sets.get(id));
    }
}

fn draw_mesh(render_pass: &mut Pass<'_>, buffer: &Buffer<Instance3dPayload>, mesh_index_count: Option<u32>) {
    let Some(index_count) = mesh_index_count else {
        panic!("2D mesh must be loaded before drawing");
    };

    let buffer = buffer.as_ref();
    if buffer.len() == 0 {
        return;
    }

    render_pass.set_vertex_buffer(1, buffer.inner().slice(..));
    render_pass.draw_indexed(0..index_count, 0, 0..buffer.len() as u32);
}
