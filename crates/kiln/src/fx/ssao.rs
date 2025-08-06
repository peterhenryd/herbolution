#![allow(dead_code)]
// TODO: finish

use std::array;

use bytemuck::cast_slice;
use prims::vector::Vec3;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::wgt::TextureDataOrder;
use wgpu::{Buffer, BufferUsages, Extent3d, Sampler, SamplerDescriptor, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use crate::Gpu;

pub struct Ssao {
    kernel_buffer: Buffer,
    noise_texture: Texture,
    noise_sampler: Sampler,
}

impl Ssao {
    pub fn new(gpu: &Gpu) -> Self {
        let kernel: [_; 64] = array::from_fn(|i| {
            let mut sample = Vec3::new(fastrand::f32() * 2.0 - 1.0, fastrand::f32() * 2.0 - 1.0, fastrand::f32());
            sample = sample.normalize();
            sample *= fastrand::f32();

            fn lerp(a: f32, b: f32, t: f32) -> f32 {
                a + (b - a) * t
            }

            let mut scale = i as f32 / 64.0;
            scale = lerp(0.1, 1.0, scale * scale);

            sample * scale
        });

        let noise: [_; 16 * 4] = array::from_fn(|i| match i % 4 {
            0 | 1 => fastrand::f32() * 2.0 - 1.0,
            _ => 0.0,
        });

        let kernel_buffer = gpu
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: cast_slice(&kernel),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            });
        let noise_texture = gpu.device.create_texture_with_data(
            &gpu.queue,
            &TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: 4,
                    height: 4,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba32Float,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            TextureDataOrder::LayerMajor,
            cast_slice(&noise),
        );
        let noise_sampler = gpu
            .device
            .create_sampler(&SamplerDescriptor::default());

        Self {
            kernel_buffer,
            noise_texture,
            noise_sampler,
        }
    }
}
