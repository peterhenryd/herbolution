use crate::engine::gpu::Gpu;
use math::to_no_uninit::ToNoUninit;
use std::ops::Deref;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, Queue};

pub struct Uniform<O> {
    queue: Queue,
    pub(crate) buffer: Buffer,
    object: O,
}

impl<O: ToNoUninit> Uniform<O> {
    pub fn create(gpu: &Gpu, name: &str, object: O) -> Self {
        let buffer = gpu.device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some(&format!("herbolution_{name}_uniform_buffer")),
                contents: bytemuck::cast_slice(&[object.to_no_uninit()]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

        Self {
            queue: gpu.queue.clone(),
            buffer,
            object,
        }
    }

    pub fn edit(&mut self, mut f: impl FnMut(&mut O)) {
        f(&mut self.object);
        self.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.object.to_no_uninit()]));
    }
}

impl<O> Deref for Uniform<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

pub trait AsByteStructUniformExt: Sized {
    fn into_uniform(self, gpu: &Gpu, name: &str) -> Uniform<Self>;
}

impl<T: ToNoUninit> AsByteStructUniformExt for T {
    fn into_uniform(self, gpu: &Gpu, name: &str) -> Uniform<Self> {
        Uniform::create(gpu, name, self)
    }
}