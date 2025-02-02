use std::sync::Arc;
use wgpu::{
    Device, LoadOp, Operations, RenderPassDepthStencilAttachment, StoreOp, SurfaceConfiguration,
    SurfaceError, SurfaceTexture, TextureUsages,
};
use winit::dpi::PhysicalSize;
use crate::gpu::Gpu;
use crate::gpu::texture::DepthTexture;

pub struct Surface<'w> {
    device: Arc<Device>,
    inner: wgpu::Surface<'w>,
    config: SurfaceConfiguration,
    depth_texture: DepthTexture,
}

impl<'w> Surface<'w> {
    pub fn new(gpu: &Gpu, inner: wgpu::Surface<'w>, size: PhysicalSize<u32>) -> Self {
        let capabilities = inner.get_capabilities(&gpu.adapter);
        let format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        let depth_texture = DepthTexture::create(&gpu.device, size);

        inner.configure(&gpu.device, &config);

        Self {
            device: gpu.device.clone(),
            inner,
            config,
            depth_texture,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.inner.configure(&self.device, &self.config);
        self.depth_texture = DepthTexture::create(&self.device, size);
    }

    pub fn get_texture_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        PhysicalSize {
            width: self.config.width,
            height: self.config.height,
        }
    }

    pub fn get_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        self.inner.get_current_texture()
    }

    pub fn depth_stencil_attachment(&self) -> RenderPassDepthStencilAttachment {
        RenderPassDepthStencilAttachment {
            view: &self.depth_texture.1,
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        }
    }
}
