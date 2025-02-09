use crate::engine::gpu::Gpu;
use wgpu::{Device, PresentMode, SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor};
use winit::dpi::PhysicalSize;

pub struct Surface(Device, wgpu::Surface<'static>, SurfaceConfiguration);

impl Surface {
    pub fn new(gpu: &Gpu, inner: wgpu::Surface<'static>, size: PhysicalSize<u32>) -> Self {
        let capabilities = inner.get_capabilities(&gpu.adapter);
        let format = capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Immediate,
            desired_maximum_frame_latency: 2,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        inner.configure(&gpu.device, &config);

        Self(gpu.device.clone(), inner, config)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let Self(device, surface, config) = self;
        config.width = size.width;
        config.height = size.height;
        surface.configure(device, config);
    }

    pub fn prepare(&self) -> (SurfaceTexture, TextureView) {
        let surface_texture = self.1.get_current_texture().expect("Failed to get surface texture");
        let texture_view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

        (surface_texture, texture_view)
    }

    pub fn get_format(&self) -> TextureFormat {
        self.2.format
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(self.2.width, self.2.height)
    }
}