use crate::engine::geometry::instance::ArrInstance;
use crate::engine::gpu::Gpu;
use bytemuck::Zeroable;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::Buffer;

pub trait ChunkMesh {
    fn schedule_update(&mut self);

    fn update(&mut self, gpu: &Gpu, instances: &[ArrInstance]);
}

#[derive(Debug, Clone)]
pub struct InstanceMesh {
    pub(crate) instance_buffer: Buffer,
    pub(crate) instance_count: u32,
    pub(crate) has_changed: bool,
}

impl InstanceMesh {
    pub fn new(gpu: &Gpu, instances: &[ArrInstance]) -> Self {
        let zeroed = [ArrInstance::zeroed()];
        let contents = if instances.is_empty() {
            bytemuck::cast_slice(&zeroed)
        } else {
            bytemuck::cast_slice(&instances)
        };

        Self {
            instance_buffer: gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            instance_count: instances.len() as u32,
            // This must be true otherwise chunk boundaries will not be culled initially.
            has_changed: true,
        }
    }
}

impl ChunkMesh for InstanceMesh {
    fn schedule_update(&mut self) {
        self.has_changed = true;
    }

    fn update(&mut self, gpu: &Gpu, instances: &[ArrInstance]) {
        self.has_changed = false;
        self.instance_count = instances.len() as u32;

        if self.instance_buffer.size() < (instances.len() * size_of::<ArrInstance>()) as u64 {
            self.instance_buffer = gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&instances),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
        } else {
            let zeroed = [ArrInstance::zeroed()];
            let contents = if instances.is_empty() {
                bytemuck::cast_slice(&zeroed)
            } else {
                bytemuck::cast_slice(&instances)
            };

            gpu.queue.write_buffer(&self.instance_buffer, 0, contents);
        }
    }
}

impl ChunkMesh for () {
    fn schedule_update(&mut self) {}

    fn update(&mut self, _: &Gpu, _: &[ArrInstance]) {}
}