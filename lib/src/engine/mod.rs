use crate::engine::gpu::Gpu;
use crate::engine::surface::Surface;
use std::sync::Arc;
use wgpu::Instance;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::listener::{InputEvent, Listener};

pub mod as_no_uninit;
pub mod binding;
pub mod buffer;
pub mod geometry;
pub mod gpu;
pub mod mesh;
pub mod pipeline;
pub mod surface;
pub mod texture;
pub mod uniform;
pub mod storage;

pub struct Engine {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub surface: Surface,
}

impl Engine {
    pub fn create(window: Arc<Window>) -> Self {
        let instance = Instance::default();
        let size = window.inner_size();
        let inner_surface = instance.create_surface(window.clone()).expect("Failed to create surface");

        let gpu = Gpu::create(&instance, &inner_surface).expect("Failed to create GPU");
        let surface = Surface::new(&gpu, inner_surface, size);

        Self { window, gpu, surface }
    }
}

impl Listener for Engine {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.surface.resize(size);
    }

    fn on_input(&mut self, _: &InputEvent) {}
}