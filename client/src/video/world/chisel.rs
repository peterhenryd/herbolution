use crate::video::frame::Frame;
use crate::video::resource::{Buffer, MeshId, SetId};
use crate::video::world::{Instance3d, RenderType, Sculptor};

pub struct Chisel<'h, 'f, 'a> {
    pub frame: &'f mut Frame<'h>,
    renderer: &'a Sculptor,
    mesh_index_count: Option<u32>,
}

impl<'h, 'f, 'a> Chisel<'h, 'f, 'a> {
    pub fn create(render_type: RenderType, frame: &'f mut Frame<'h>, renderer: &'a Sculptor) -> Self {
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

    pub fn render_each(&mut self, buffer: impl AsRef<Buffer<Instance3d>>) {
        let Some(index_count) = self.mesh_index_count else {
            panic!("2D mesh must be loaded before drawing");
        };

        let buffer = buffer.as_ref();
        if buffer.len() == 0 {
            return;
        }

        self.frame
            .pass()
            .set_vertex_buffer(1, buffer.inner().slice(..));
        self.frame
            .pass()
            .draw_indexed(0..index_count, 0, 0..buffer.len() as u32);
    }

    pub fn render_each_by_id(&mut self, id: SetId) {
        self.render_each(self.renderer.sets.get(id));
    }
}
