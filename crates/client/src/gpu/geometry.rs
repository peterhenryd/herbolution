use crate::gpu::binding::Payload;
use bytemuck::{cast_slice, NoUninit, Zeroable};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferAddress, BufferUsages, VertexAttribute, VertexBufferLayout};
use crate::gpu::Gpu;

#[derive(Debug)]
pub struct Mesh {
    pub(crate) vertex_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
    pub(crate) index_count: u32,
}

pub trait Primitive: Payload {
    const LAYOUT: VertexBufferLayout<'static>;
    const ATTRIBUTES: &'static [VertexAttribute];
}

#[derive(Debug)]
pub struct InstanceBuffer {
    pub(crate) buffer: Buffer,
    pub(crate) count: u32,
}

impl Mesh {
    pub fn create(gpu: &Gpu, vertices: &[impl NoUninit], indices: &[u16]) -> Self {
        Self {
            vertex_buffer: gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(vertices),
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                }),
            index_buffer: gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(indices),
                    usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
                }),
            index_count: indices.len() as u32,
        }
    }
}

impl InstanceBuffer {
    pub fn create<T>(gpu: &Gpu, instances: &[T]) -> Self
    where T: NoUninit + Zeroable {
        let count = instances.len() as u32;
        let buffer;
        if instances.is_empty() {
            buffer = gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(&[T::zeroed()]),
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                });
        } else {
            buffer = gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(instances),
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                });
        }

        Self { buffer, count }
    }

    pub fn write<T>(&mut self, gpu: &Gpu, instances: &[T])
    where T: NoUninit + Zeroable {
        if (instances.len() * size_of::<T>()) as BufferAddress > self.buffer.size() {
            self.buffer = gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(instances),
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                });
        } else {
            gpu.queue.write_buffer(&self.buffer, 0, cast_slice(instances));
        }
        self.count = instances.len() as u32;
    }
}

impl AsRef<Buffer> for InstanceBuffer {
    fn as_ref(&self) -> &Buffer {
        &self.buffer
    }
}