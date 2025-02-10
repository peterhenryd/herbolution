use crate::engine::gpu::Gpu;
use std::ops::Deref;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, Device, Queue};
use math::as_no_uninit::AsNoUninit;

pub struct Storage<O> {
    device: Device,
    queue: Queue,
    pub(crate) buffer: Buffer,
    objects: Vec<O>,
}

impl<O: AsNoUninit> Storage<O> {
    pub fn create(gpu: &Gpu, name: &str, objects: Vec<O>) -> Self {
        let byte_objects = objects.iter()
            .map(|x| x.as_no_uninit())
            .collect::<Vec<_>>();
        let buffer = gpu.device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some(&format!("herbolution_{name}_storage_buffer")),
                contents: bytemuck::cast_slice(&byte_objects),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            });

        Self {
            device: gpu.device.clone(),
            queue: gpu.queue.clone(),
            buffer,
            objects,
        }
    }

    pub fn edit(&mut self, mut f: impl FnMut(&mut Vec<O>)) {
        f(&mut self.objects);
        let byte_objects = self.objects.iter()
            .map(|x| x.as_no_uninit())
            .collect::<Vec<_>>();
        if self.buffer.size() < (byte_objects.len() * size_of::<O>()) as u64 {
            self.buffer = self.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&byte_objects),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                });
        } else {
            self.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&byte_objects));
        }
    }
}

impl<O> Deref for Storage<O> {
    type Target = Vec<O>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}