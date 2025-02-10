use crate::engine::gpu::Gpu;
use crate::engine::mesh::VertexIndex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::Buffer;
use math::as_no_uninit::AsNoUninit;

impl Gpu {
    pub fn create_vertex_buffer<V: AsNoUninit>(&self, vertices: &[V]) -> Buffer {
        let vertices = vertices.iter()
            .map(|x| x.as_no_uninit())
            .collect::<Vec<_>>();
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub fn create_index_buffer<I: VertexIndex>(&self, indices: &[I]) -> Buffer {
        let indices = indices.iter()
            .map(|x| x.as_no_uninit())
            .collect::<Vec<_>>();
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        })
    }
}