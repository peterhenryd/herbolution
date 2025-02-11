use crate::engine::gpu::Gpu;
use crate::engine::mesh::VertexIndex;
use math::to_no_uninit::ToNoUninit;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::Buffer;

impl Gpu {
    pub fn create_vertex_buffer<V: ToNoUninit>(&self, vertices: &[V]) -> Buffer {
        let vertices = vertices.iter()
            .map(|x| x.to_no_uninit())
            .collect::<Vec<_>>();
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub fn create_index_buffer<I: VertexIndex>(&self, indices: &[I]) -> Buffer {
        let indices = indices.iter()
            .map(|x| x.to_no_uninit())
            .collect::<Vec<_>>();
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        })
    }
}