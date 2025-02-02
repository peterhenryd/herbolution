use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BufferBindingType, BufferUsages, Queue, ShaderStages};
use crate::gpu::buffer::Buffer;
use crate::gpu::Gpu;
use crate::gpu::uniform::UniformObject;

pub struct Uniform<U> {
    queue: Arc<Queue>,
    buffer: wgpu::Buffer,
    value: U,
    visibility: ShaderStages,
}

impl<U: UniformObject> Uniform<U> {
    pub fn new(gpu: &Gpu, name: impl Into<String>, value: U, visibility: ShaderStages) -> Self {
        Self {
            queue: gpu.queue.clone(),
            buffer: gpu.device.create_buffer_init(&BufferInitDescriptor {
                label: Some(&format!("herbolution_{}_uniform_buffer", name.into())),
                contents: bytemuck::cast_slice(&[value.get_raw()]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            }),
            value,
            visibility,
        }
    }

    pub fn write_changes(&self) {
        self.queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&[self.value.get_raw()]),
        );
    }
}

impl<U> AsRef<wgpu::Buffer> for Uniform<U> {
    fn as_ref(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

impl<U> Buffer for Uniform<U> {
    fn binding_type(&self) -> BufferBindingType {
        BufferBindingType::Uniform
    }

    fn visibility(&self) -> ShaderStages {
        self.visibility
    }
}

impl<U> Deref for Uniform<U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<U> DerefMut for Uniform<U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
