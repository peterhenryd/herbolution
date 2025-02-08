use crate::engine::gpu::Gpu;
use std::ops::Deref;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, Queue};
use crate::engine::as_no_uninit::AsNoUninit;

pub struct Storage<O> {
    queue: Queue,
    pub(crate) buffer: Buffer,
    object: O,
}

impl<O: AsNoUninit> Storage<O> {
    pub fn create(gpu: &Gpu, name: &str, object: O) -> Self {
        let buffer = gpu.device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some(&format!("herbolution_{name}_storage_buffer")),
                contents: bytemuck::cast_slice(&[object.as_no_uninit()]),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            });

        Self {
            queue: gpu.queue.clone(),
            buffer,
            object,
        }
    }

    pub fn edit(&mut self, mut f: impl FnMut(&mut O)) {
        f(&mut self.object);
        self.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.object.as_no_uninit()]));
    }
}

impl<O> Deref for Storage<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

pub trait AsByteStructStorageExt: Sized {
    fn into_storage(self, gpu: &Gpu, name: &str) -> Storage<Self>;
}

impl<T: AsNoUninit> AsByteStructStorageExt for T {
    fn into_storage(self, gpu: &Gpu, name: &str) -> Storage<Self> {
        Storage::create(gpu, name, self)
    }
}