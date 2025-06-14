use gpu::frame::{Frame, Pass};
use gpu::{Buffer, SetId, MeshId};
use crate::{RenderType, Renderer};
use crate::vertex::Instance2dPayload;

pub struct Drawing<'q, 'f, 'r> {
    frame: &'f mut Frame<'q>,
    renderer: &'r Renderer,
    mesh_index_count: Option<u32>,
}

impl<'q, 'f, 'r> Drawing<'q, 'f, 'r> {
    pub fn create(render_type: RenderType, frame: &'f mut Frame<'q>, renderer: &'r Renderer) -> Self {
        renderer.pipeline_map.load_into_render_pass(render_type, frame.pass());
        
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

    pub fn draw(&mut self, buffer: &Buffer<Instance2dPayload>) {
        draw_mesh(self.frame.pass(), buffer, self.mesh_index_count);
    }

    pub fn draw_from_set(&mut self, id: SetId) {
        self.draw(self.renderer.instance_sets.get(id));
    }
}

fn draw_mesh(pass: &mut Pass<'_>, buffer: &Buffer<Instance2dPayload>, mesh_index_count: Option<u32>) {
    let Some(index_count) = mesh_index_count else {
        panic!("2D mesh must be loaded before drawing");
    };

    let buffer = buffer.as_ref();
    if buffer.len() == 0 {
        return;
    }

    pass.set_vertex_buffer(1, buffer.inner.slice(..));
    pass.draw_indexed(0..index_count, 0, 0..buffer.len() as u32);
}