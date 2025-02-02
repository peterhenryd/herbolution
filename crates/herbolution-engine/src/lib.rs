extern crate herbolution_math as math;

use crate::gpu::surface::Surface;
use crate::gpu::{CreateGpuError, Gpu};
use std::sync::Arc;
use thiserror::Error;
use wgpu::{CreateSurfaceError, Instance};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::ui::UiRenderer;
use crate::world::WorldRenderer;

pub mod camera;
pub mod gpu;
pub mod input;
pub mod world;
pub mod ui;

pub struct Engine<'w> {
    // Operating system handles
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub surface: Surface<'w>,
    pub ui: UiRenderer,
    pub world: WorldRenderer,
}

#[derive(Debug, Error)]
pub enum CreateEngineError {
    #[error(transparent)]
    CreateSurface(#[from] CreateSurfaceError),
    #[error(transparent)]
    CreateGpu(#[from] CreateGpuError),
}

impl<'w> Engine<'w> {
    pub async fn create(window: Arc<Window>) -> Result<Self, CreateEngineError> {
        let instance = Instance::default();
        let inner_surface = instance.create_surface(window.clone())?;
        let gpu = Gpu::create(&instance, &inner_surface).await?;
        let surface = Surface::new(&gpu, inner_surface, window.inner_size());

        Ok(Self {
            window,
            gpu,
            surface,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface.resize(size);
    }
}