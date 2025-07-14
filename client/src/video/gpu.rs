use lib::size::{Size2, size2u};
use pollster::FutureExt;
use wgpu::{
    Adapter, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterOptions,
    SurfaceConfiguration, TextureFormat, TextureUsages, TextureView, Trace,
};

use crate::video::resource::{SampleCount, Texture};

#[derive(Debug, Clone)]
pub struct Handle {
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl Handle {
    #[inline]
    pub fn new_unchecked(adapter: Adapter, device: Device, queue: Queue) -> Self {
        Self { adapter, device, queue }
    }

    pub fn create(instance: &Instance, surface: &wgpu::Surface<'_>) -> Self {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .expect("Failed to request adapter");
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .block_on()
            .expect("Failed to request device");

        Self { adapter, device, queue }
    }

    #[inline]
    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    #[inline]
    pub fn device(&self) -> &Device {
        &self.device
    }

    #[inline]
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

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
            alpha_mode: CompositeAlphaMode::Opaque,
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

pub fn create<'w>(target: impl Into<wgpu::SurfaceTarget<'w>>, resolution: impl Into<size2u>, sample_count: SampleCount) -> (Handle, Surface<'w>) {
    let instance = wgpu::Instance::default();
    let surface = instance
        .create_surface(target)
        .expect("Failed to create surface");

    let handle = Handle::create(&instance, &surface);
    let surface = Surface::new(&handle, surface, resolution.into(), sample_count);

    (handle, surface)
}
