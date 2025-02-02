use crate::engine::gpu::Gpu;
use bytemuck::NoUninit;
use std::ops::Range;
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, IndexFormat, Queue, RenderPass};

pub struct Model {
    queue: Arc<Queue>,
    pub(crate) mesh_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
    index_count: u32,
}

impl Model {
    pub fn new(gpu: &Gpu, vertices: &[impl NoUninit], indices: &[u16]) -> Self {
        let queue = gpu.queue.clone();
        let mesh_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });
        let index_count = indices.len() as u32;

        Self {
            queue,
            mesh_buffer,
            index_buffer,
            index_count,
        }
    }

    pub fn update(&mut self, vertices: &[impl NoUninit], indices: &[u16]) {
        self.queue
            .write_buffer(&self.mesh_buffer, 0, bytemuck::cast_slice(vertices));
        self.queue
            .write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(indices));
        self.index_count = indices.len() as u32;
    }

    pub fn draw(&self, render_pass: &mut RenderPass, instances: Range<u32>) {
        render_pass.set_vertex_buffer(0, self.mesh_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_count, 0, instances);
    }
}
