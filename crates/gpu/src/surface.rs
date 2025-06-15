use math::size::Size2;
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
    pub fn new(handle: &Handle, inner: wgpu::Surface<'w>, resolution: impl Into<Size2<u32>>) -> Self {
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
        let depth_texture = Texture::depth(handle, resolution);

        inner.configure(handle.device(), &config);

        Self { inner, config, depth_texture }
    }

    pub fn set_resolution(&mut self, handle: &Handle, resolution: impl Into<Size2<u32>>) {
        let resolution = resolution.into();
        self.config.width = resolution.width;
        self.config.height = resolution.height;
        self.inner
            .configure(handle.device(), &self.config);

        self.depth_texture = Texture::depth(handle, resolution);
    }

    pub fn resolution(&self) -> Size2<u32> {
        Size2::new(self.config.width, self.config.height)
    }
}
