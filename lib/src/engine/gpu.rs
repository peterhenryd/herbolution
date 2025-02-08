use pollster::FutureExt;
use thiserror::Error;
use wgpu::*;

#[derive(Debug, Clone)]
pub struct Gpu {
    pub(crate) adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

#[derive(Debug, Error)]
pub enum CreateGpuError {
    #[error("Failed to find a suitable GPU adapter")]
    RequestAdapter,
    #[error(transparent)]
    RequestDevice(#[from] RequestDeviceError),
}

impl Gpu {
    const FEATURES: Features = Features::TEXTURE_BINDING_ARRAY;

    pub fn create(instance: &Instance, surface: &Surface) -> Result<Self, CreateGpuError> {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .block_on()
            .ok_or(CreateGpuError::RequestAdapter)?;
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Herbolution GPU Device"),
                required_features: Self::FEATURES,
                required_limits: Limits::default(),
                memory_hints: MemoryHints::MemoryUsage,
            }, None)
            .block_on()?;

        Ok(Self { adapter, device, queue })
    }
}