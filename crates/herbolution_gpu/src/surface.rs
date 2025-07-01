use math::size::{size2u, Size2};
pub use wgpu::SurfaceTarget;
use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages, TextureView};

use crate::handle::Handle;
use crate::texture::{SampleCount, Texture};

pub struct Surface<'w> {
    inner: wgpu::Surface<'w>,
    config: SurfaceConfiguration,
    depth_texture: Texture,
    sample_count: SampleCount,
}

impl<'w> Surface<'w> {
    pub fn new(gpu: &Handle, inner: wgpu::Surface<'w>, resolution: size2u, sample_count: SampleCount) -> Self {
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
        let depth_texture = Texture::depth(gpu, resolution, sample_count);

        inner.configure(gpu.device(), &config);

        Self {
            inner,
            config,
            depth_texture,
            sample_count,
        }
    }

    pub fn set_sample_count(&mut self, gpu: &Handle, sample_count: SampleCount) {
        self.sample_count = sample_count;
        self.depth_texture = Texture::depth(gpu, self.resolution(), sample_count);
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }

    pub fn set_resolution(&mut self, gpu: &Handle, resolution: impl Into<size2u>) {
        let resolution = resolution.into();
        self.config.width = resolution.width;
        self.config.height = resolution.height;
        self.inner.configure(gpu.device(), &self.config);
        self.depth_texture = Texture::depth(gpu, resolution, self.sample_count);
    }

    pub fn resolution(&self) -> size2u {
        Size2::new(self.config.width, self.config.height)
    }

    pub fn depth_texture(&self) -> &Texture {
        &self.depth_texture
    }

    pub fn create_texture(&self) -> Option<SurfaceTexture> {
        let inner = self.inner.get_current_texture().ok()?;
        let view = inner.texture.create_view(&Default::default());

        Some(SurfaceTexture { inner: Some(inner), view })
    }
}

pub struct SurfaceTexture {
    inner: Option<wgpu::SurfaceTexture>,
    view: TextureView,
}

impl SurfaceTexture {
    pub fn resolution(&self) -> size2u {
        let inner = self.inner.as_ref().unwrap();
        Size2::new(inner.texture.width(), inner.texture.height())
    }

    pub fn format(&self) -> TextureFormat {
        let inner = self.inner.as_ref().unwrap();
        inner.texture.format()
    }
}

impl AsRef<TextureView> for SurfaceTexture {
    fn as_ref(&self) -> &TextureView {
        &self.view
    }
}

impl Drop for SurfaceTexture {
    fn drop(&mut self) {
        if let Some(texture) = self.inner.take() {
            texture.present();
        }
    }
}
