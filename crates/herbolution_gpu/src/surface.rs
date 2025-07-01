use math::size::{size2u, Size2};
pub use wgpu::SurfaceTarget as Target;
use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages};

use crate::handle::Handle;
use crate::texture::Texture;

pub struct Surface<'w> {
    pub(crate) inner: wgpu::Surface<'w>,
    config: SurfaceConfiguration,
    pub(crate) depth_texture: Texture,
}

impl<'w> Surface<'w> {
    pub fn new(gpu: &Handle, inner: wgpu::Surface<'w>, resolution: impl Into<size2u>) -> Self {
        let resolution = resolution.into();
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: resolution.width,
            height: resolution.height,
            present_mode: PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: CompositeAlphaMode::PostMultiplied,
            view_formats: vec![],
        };
        let depth_texture = Texture::depth(gpu, resolution);

        inner.configure(gpu.device(), &config);

        Self { inner, config, depth_texture }
    }

    pub fn set_resolution(&mut self, gpu: &Handle, resolution: impl Into<size2u>) {
        let resolution = resolution.into();
        self.config.width = resolution.width;
        self.config.height = resolution.height;
        self.inner.configure(gpu.device(), &self.config);

        self.depth_texture = Texture::depth(gpu, resolution);
    }

    pub fn resolution(&self) -> size2u {
        Size2::new(self.config.width, self.config.height)
    }
}
