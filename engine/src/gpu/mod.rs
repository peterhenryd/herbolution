use crate::gpu::frame::GpuFrame;
use crate::gpu::handle::Handle;
use crate::gpu::mem::texture::DepthTexture;
use crate::gpu::surface::Surface;
use math::size::Size2;
use std::sync::Arc;
use thiserror::Error;
pub use wgpu::{CreateSurfaceError, RequestDeviceError, SurfaceError, TextureFormat};
use wgpu::Instance;
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub mod frame;
pub mod handle;
pub mod mem;
pub mod surface;

pub struct Gpu {
    pub handle: Handle,
    pub surface: Surface,
    depth_texture: DepthTexture,
}

impl Gpu {
    pub fn create(window: Arc<Window>) -> Result<Self, GpuError> {
        let PhysicalSize { width, height } = window.inner_size();
        let size = Size2::new(width, height);

        let instance = Instance::default();
        let surface = instance.create_surface(window)?;

        let handle = Handle::create(&instance, &surface)?;
        let surface = Surface::new(&handle, surface, size);
        let depth_texture = DepthTexture::create(&handle, size);

        Ok(Self { handle, surface, depth_texture })
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.surface.set_size(&self.handle, size);
        self.depth_texture.set_size(&self.handle, size);
    }

    pub fn size(&self) -> Size2<u32> {
        self.surface.size()
    }

    pub fn create_frame(&self) -> Result<GpuFrame, GpuError> {
        GpuFrame::create(self)
    }
}

#[derive(Debug, Error)]
pub enum GpuError {
    #[error(transparent)]
    CreateSurface(#[from] CreateSurfaceError),
    #[error("Failed to find a suitable GPU adapter")]
    RequestAdapter,
    #[error(transparent)]
    RequestDevice(#[from] RequestDeviceError),
    #[error(transparent)]
    Surface(#[from] SurfaceError),
}