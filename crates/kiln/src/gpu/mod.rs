pub(crate) mod error;
pub(crate) mod surface;

use pollster::FutureExt;

#[derive(Debug)]
pub struct Gpu {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl Gpu {
    #[inline]
    pub fn new() -> error::GpuResult<Self> {
        Self::for_surface(None)
    }

    pub fn for_surface(compatible_surface: Option<&wgpu::Surface<'_>>) -> error::GpuResult<Self> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface,
                force_fallback_adapter: false,
                power_preference: wgpu::PowerPreference::HighPerformance,
            })
            .block_on()?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Kiln GPU Device"),
                memory_hints: wgpu::MemoryHints::Performance,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
            })
            .block_on()?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    #[inline]
    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    #[inline]
    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    #[inline]
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    #[inline]
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}
