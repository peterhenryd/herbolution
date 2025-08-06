use prims::size::{Size2, size2u};

use crate::Gpu;

pub type SurfaceTarget = wgpu::SurfaceTarget<'static>;

#[derive(Debug, PartialEq)]
pub struct Surface {
    raw: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

impl Surface {
    #[inline]
    pub fn new_unchecked(raw: wgpu::Surface<'static>, config: wgpu::SurfaceConfiguration) -> Self {
        Self { raw, config }
    }

    pub fn new(gpu: &Gpu, target: impl Into<SurfaceTarget>, size: size2u) -> Result<Self, wgpu::CreateSurfaceError> {
        let surface = gpu.instance.create_surface(target)?;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
            view_formats: vec![],
        };

        surface.configure(&gpu.device, &config);

        Ok(Self { raw: surface, config })
    }

    pub fn get_texture(&self) -> Result<SurfaceTexture, wgpu::SurfaceError> {
        let raw = self.raw.get_current_texture()?;
        let view = raw.texture.create_view(&Default::default());

        Ok(SurfaceTexture { raw, view })
    }

    pub fn set_size(&mut self, gpu: &Gpu, size: size2u) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.raw.configure(&gpu.device, &self.config);
    }

    pub fn size(&self) -> size2u {
        Size2::new(self.config.width, self.config.height)
    }
}

impl AsRef<wgpu::Surface<'static>> for Surface {
    fn as_ref(&self) -> &wgpu::Surface<'static> {
        &self.raw
    }
}

#[derive(Debug)]
pub struct SurfaceTexture {
    raw: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
}

impl SurfaceTexture {
    pub fn present(self) {
        self.raw.present();
    }

    pub fn inner(&self) -> &wgpu::SurfaceTexture {
        &self.raw
    }
}

impl AsRef<wgpu::TextureView> for SurfaceTexture {
    fn as_ref(&self) -> &wgpu::TextureView {
        &self.view
    }
}
