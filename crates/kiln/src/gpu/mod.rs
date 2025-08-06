mod buffer;
mod encoder;
mod pipeline;
mod render_pass;
mod surface;
mod texture;

pub use buffer::*;
pub use encoder::*;
pub use pipeline::*;
use pollster::FutureExt;
pub use render_pass::*;
pub use surface::*;
pub use texture::*;
use wgpu::{Adapter, Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, Queue, RequestAdapterOptions, Trace};

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Gpu {
    instance: Instance,
    adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Gpu {
    pub fn new(instance: Instance, compatible_surface: Option<&wgpu::Surface<'_>>) -> Result<Self, Error> {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface,
            })
            .block_on()?;
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .block_on()?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}

impl Default for Gpu {
    fn default() -> Self {
        Self::new(Default::default(), None).expect("Failed to create Gpu")
    }
}
