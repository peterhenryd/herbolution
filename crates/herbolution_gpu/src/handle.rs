use pollster::FutureExt;
use wgpu::{Adapter, Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, Queue, RequestAdapterOptions, Trace};

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
