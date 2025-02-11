use crate::engine::gpu::Gpu;
use math::to_no_uninit::ToNoUninit;
use wgpu::{Buffer, IndexFormat, RenderPass};

pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl Mesh {
    pub fn create<V: ToNoUninit, I: VertexIndex>(gpu: &Gpu, vertices: &[V], indices: &[I]) -> Self {
        let vertex_buffer = gpu.create_vertex_buffer(vertices);
        let index_buffer = gpu.create_index_buffer(indices);
        let index_count = indices.len() as u32;

        Self { vertex_buffer, index_buffer, index_count }
    }

    pub fn load(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
    }

    pub fn draw(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

pub trait VertexIndex: ToNoUninit {
    const FORMAT: IndexFormat;
}

impl VertexIndex for u16 {
    const FORMAT: IndexFormat = IndexFormat::Uint16;
}

impl VertexIndex for u32 {
    const FORMAT: IndexFormat = IndexFormat::Uint32;
}