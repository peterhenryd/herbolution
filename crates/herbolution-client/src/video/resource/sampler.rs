use crate::video::gpu::Handle;
use wgpu::{AddressMode, FilterMode, Sampler, SamplerDescriptor};

pub struct SamplerOptions {
    pub filter: Filter,
}

pub enum Filter {
    Pixelated,
    Smooth,
}

impl Handle {
    pub fn create_sampler(&self, options: SamplerOptions) -> Sampler {
        self.device().create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: match options.filter {
                Filter::Pixelated => FilterMode::Nearest,
                Filter::Smooth => FilterMode::Linear,
            },
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        })
    }
}
