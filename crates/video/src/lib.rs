use math::size::Size2;
pub use wgpu::SurfaceTarget;
use crate::r2d::Renderer2d;
use crate::text::TextRenderer;
 
mod camera; 
mod frame; 
mod gpu;
pub mod mem;
pub mod pipeline;
pub mod r2d;
pub mod r3d;
pub mod text;

pub use camera::{Camera, CameraPayload};
pub use frame::{Frame, Frame2d, Frame3d};
pub use gpu::{Handle, Surface};
use math::color::Rgba;
use crate::r3d::Renderer3d;

pub struct Video<'w> {
    pub handle: Handle,
    surface: Surface<'w>,
    pub text: TextRenderer,
    pub r2d: Renderer2d,
    pub r3d: Renderer3d,
    clear_color: Rgba<f64>,
}

pub struct Options {
    pub r2d: r2d::Options,
    pub r3d: r3d::Options,
    pub clear_color: Rgba<f64>,
}

impl<'w> Video<'w> {
    pub fn create(target: impl Into<SurfaceTarget<'w>>, resolution: impl Into<Size2<u32>>, options: Options) -> Self {
        let resolution = resolution.into();
        let (handle, surface) = gpu::create(target, resolution);
        let text = TextRenderer::create(&handle, resolution);
        let r2d = Renderer2d::create(&handle, options.r2d);
        let r3d = Renderer3d::create(&handle, options.r3d);
        
        Self { handle, surface, text, r2d, r3d, clear_color: options.clear_color }
    }
    
    pub fn set_resolution(&mut self, resolution: impl Into<Size2<u32>>) {
        let resolution = resolution.into();
        self.surface.set_resolution(&self.handle, resolution);
        self.text.set_resolution(&self.handle, resolution)
    }
    
    pub fn resolution(&self) -> Size2<u32> {
        self.surface.resolution()
    }
    
    pub fn create_frame(&mut self) -> Frame {
        Frame::create(self)
    }
}