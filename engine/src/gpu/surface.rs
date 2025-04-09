use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages};
pub use wgpu::SurfaceTexture;
use math::size::Size2;
use crate::gpu::GpuError;
use crate::gpu::handle::Handle;

pub struct Surface {
    inner: wgpu::Surface<'static>,
    config: SurfaceConfiguration,
}

impl Surface {
    pub fn new(handle: &Handle, inner: wgpu::Surface<'static>, size: Size2<u32>) -> Self {
        let capabilities = inner.get_capabilities(&handle.adapter);
        let format = capabilities.formats
            .iter()
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
            alpha_mode: CompositeAlphaMode::PostMultiplied,
            view_formats: vec![],
        };

        inner.configure(&handle.device, &config);

        Self {
            inner,
            config,
        }
    }

    pub fn set_size(&mut self, handle: &Handle, size: Size2<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.inner.configure(&handle.device, &self.config);
    }

    pub fn texture(&self) -> Result<SurfaceTexture, GpuError> {
        self.inner.get_current_texture().map_err(GpuError::Surface)
    }

    pub fn format(&self) -> TextureFormat {
        self.config.format
    }

    pub fn size(&self) -> Size2<u32> {
        Size2::new(self.config.width, self.config.height)
    }
}
