use crate::engine::gpu::Gpu;
use crate::engine::surface::Surface;
use std::sync::Arc;
use wgpu::Instance;
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::keyboard::KeyCode;
use winit::window::{CursorGrabMode, Window};
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
    pub is_focused: bool,
}

impl Engine {
    pub fn create(window: Arc<Window>) -> Self {
        let instance = Instance::default();
        let size = window.inner_size();
        let inner_surface = instance.create_surface(window.clone()).expect("Failed to create surface");

        let gpu = Gpu::create(&instance, &inner_surface).expect("Failed to create GPU");
        let surface = Surface::new(&gpu, inner_surface, size);

        Self { window, gpu, surface, is_focused: false }
    }
}

impl Listener for Engine {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.surface.resize(size);
    }

    fn on_input(&mut self, event: &InputEvent) {
        use InputEvent::*;
        match event {
            Key { code: KeyCode::Escape, state: ElementState::Pressed } if self.is_focused => {
                self.window.set_cursor_visible(true);
                self.window.set_cursor_grab(CursorGrabMode::None).expect("Failed to release cursor");
                self.is_focused = false;
            }
            MouseClick { state: ElementState::Pressed, .. } if !self.is_focused => {
                self.window.set_cursor_visible(false);
                self.window.set_cursor_grab(CursorGrabMode::Locked).expect("Failed to lock cursor");
                self.is_focused = true;
            }
            _ => {}
        }
    }
}