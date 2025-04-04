#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(iterator_try_collect)]

use crate::gpu::frame::GpuFrame;
use crate::gpu::{Gpu, GpuError};
use crate::input::{Input, InputFrame};
use crate::renderer_2d::Renderer2D;
use crate::renderer_3d::Renderer3D;
use math::size::Size2;
use std::sync::Arc;
use winit::window::Window;

pub mod camera;
pub mod gpu;
pub mod input;
pub mod renderer_2d;
pub mod renderer_3d;

pub struct Engine {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub input: Input,
    pub renderer_2d: Renderer2D,
    pub renderer_3d: Renderer3D,
}

impl Engine {
    pub fn create(window: Arc<Window>) -> Result<Self, GpuError> {
        let gpu = Gpu::create(window.clone())?;
        let input = Input::default();

        let (size, format) = (gpu.surface.size(), gpu.surface.format());
        let renderer_2d = Renderer2D::create(&gpu.handle, size, format);
        let renderer_3d = Renderer3D::create(&gpu.handle, size, format);

        Ok(Self {
            window,
            gpu,
            input,
            renderer_2d,
            renderer_3d,
        })
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.gpu.set_size(size);
        self.renderer_2d.set_size(&self.gpu.handle, size);
        self.renderer_3d.set_size(size);
    }

    pub fn create_frame(&mut self) -> Result<EngineFrame, GpuError> {
        Ok(EngineFrame {
            gpu: self.gpu.create_frame()?,
            input: self.input.take_frame(),
        })
    }

    pub fn update(&mut self) {
        self.renderer_2d.update(&self.gpu.handle);
        self.renderer_3d.update(&self.gpu.handle);
    }

    /*
    pub fn on_input(&mut self, event: &InputEvent) {
        use InputEvent::*;
        match event {
            Key {
                code: KeyCode::Escape,
                state: ElementState::Pressed,
            } if self.is_focused => {
                self.window.set_cursor_visible(true);
                self.window
                    .set_cursor_grab(CursorGrabMode::None)
                    .expect("Failed to release cursor");
                self.is_focused = false;
            }
            MouseClick {
                state: ElementState::Pressed,
                ..
            } if !self.is_focused => {
                self.window.set_cursor_visible(false);
                self.window
                    .set_cursor_grab(CursorGrabMode::Locked)
                    .expect("Failed to lock cursor");
                self.is_focused = true;
            }
            _ => {}
        }
    }

     */
}

pub struct EngineFrame {
    pub gpu: GpuFrame,
    pub input: InputFrame,
}