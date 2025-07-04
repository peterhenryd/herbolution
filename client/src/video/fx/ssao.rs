use std::array;

use lib::random;
use lib::vector::{vec3f, Vec3};
use wgpu::{BufferUsages, Sampler, TextureFormat};

use crate::video::gpu::{Buffer, Filter, Handle, SamplerOptions, Texture};

pub struct Ssao {
    kernel_buffer: Buffer<vec3f>,
    noise_texture: Texture,
    noise_sampler: Sampler,
}

impl Ssao {
    pub fn new(gpu: &Handle) -> Self {
        let kernel: [_; 64] = array::from_fn(|i| {
            let mut sample = Vec3::new(random::f32() * 2.0 - 1.0, random::f32() * 2.0 - 1.0, random::f32());
            sample = sample.normalize();
            sample *= random::f32();

            fn lerp(a: f32, b: f32, t: f32) -> f32 {
                a + (b - a) * t
            }

            let mut scale = i as f32 / 64.0;
            scale = lerp(0.1, 1.0, scale * scale);

            sample * scale
        });

        let noise: [_; 16 * 4] = array::from_fn(|i| match i % 4 {
            0 | 1 => random::f32() * 2.0 - 1.0,
            _ => 0.0,
        });

        let kernel_buffer = Buffer::from_data(gpu, &kernel, BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        let noise_texture = Texture::from_data(gpu, (4, 4), TextureFormat::Rgba32Float, noise.as_slice());
        let noise_sampler = gpu.create_sampler(SamplerOptions { filter: Filter::Smooth });

        Self {
            kernel_buffer,
            noise_texture,
            noise_sampler,
        }
    }
}
