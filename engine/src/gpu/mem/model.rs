use bytemuck::{NoUninit, Zeroable};
use crate::gpu::handle::Handle;
use wgpu::{Buffer, IndexFormat, RenderPass, VertexAttribute, VertexBufferLayout};
use crate::gpu::mem::payload::ShaderPayload;

pub trait VertexShaderArgument: ShaderPayload {
    const ATTRIBUTES: &'static [VertexAttribute];
    const LAYOUT: VertexBufferLayout<'static>;
}

pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl Mesh {
    pub fn create(handle: &Handle, vertices: &[impl VertexShaderArgument], indices: &[u16]) -> Self {
        let vertex_payloads = vertices.iter().map(|vertex| vertex.payload()).collect::<Vec<_>>();
        Self {
            vertex_buffer: handle.create_vertex_buffer(&vertex_payloads),
            index_buffer: handle.create_index_buffer(indices),
            index_count: indices.len() as u32,
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>, instance_groups: impl Iterator<Item = &InstanceGroup>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);

        for instance in instance_groups {
            render_pass.set_vertex_buffer(1, instance.buffer.slice(..));
            render_pass.draw_indexed(0..self.index_count, 0, 0..instance.count);
        }
    }
}

#[derive(Debug)]
pub struct InstanceGroup {
    buffer: Buffer,
    count: u32,
}

impl InstanceGroup {
    pub fn create<T: NoUninit + Zeroable>(handle: &Handle, instances: &[T]) -> Self {
        if instances.is_empty() {
            return Self {
                buffer: handle.create_vertex_buffer(&[T::zeroed()]),
                count: 0,
            };
        }

        Self {
            buffer: handle.create_vertex_buffer(&instances),
            count: instances.len() as u32,
        }
    }

    pub fn write<T: NoUninit + Zeroable>(&mut self, handle: &Handle, instances: &[T]) {
        if instances.len() as u32 > self.count {
            self.buffer = handle.create_vertex_buffer(&instances);
            self.count = instances.len() as u32;
        } else {
            handle.write_buffer(&self.buffer, 0, &instances);
        }
    }
}