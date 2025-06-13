use pollster::FutureExt;
use wgpu::{Adapter, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterOptions, SurfaceConfiguration, SurfaceTarget, TextureFormat, TextureUsages, Trace};
use math::size::Size2;
use crate::mem::texture::Texture;

pub use wgpu::include_wgsl;

#[derive(Debug)]
pub struct Handle {
    adapter: Adapter, 
    device: Device,
    queue: Queue,
}

impl Handle {
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
                trace: Trace::Off
            })
            .block_on()
            .expect("Failed to request device");
        
        Self {
            adapter,
            device,
            queue,
        }
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
    
    pub(crate) fn clone(handle: &Self) -> Self {
        Self {
            adapter: handle.adapter.clone(),
            device: handle.device.clone(),
            queue: handle.queue.clone(),
        }
    }
}

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
            present_mode: PresentMode::Immediate,
            desired_maximum_frame_latency: 2,
            alpha_mode: CompositeAlphaMode::Auto,
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
        self.inner.configure(handle.device(), &self.config);

        self.depth_texture = Texture::depth(handle, resolution);
    }
    
    pub fn resolution(&self) -> Size2<u32> {
        Size2::new(self.config.width, self.config.height)
    }
}

pub fn create<'w>(target: impl Into<SurfaceTarget<'w>>, resolution: impl Into<Size2<u32>>) -> (Handle, Surface<'w>) {
    let instance = Instance::default();
    let surface = instance
        .create_surface(target)
        .expect("Failed to create surface");
    
    let handle = Handle::create(&instance, &surface);
    let surface = Surface::new(&handle, surface, resolution);

    (handle, surface)
}

#[macro_export]
macro_rules! load_shader {
    ($handle:expr, $path:literal) => {
        ($handle).device().create_shader_module($crate::gpu::include_wgsl!($path))
    };
}
