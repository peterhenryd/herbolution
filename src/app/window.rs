use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::error::OsError;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;

#[derive(Clone)]
pub struct Window(Arc<winit::window::Window>);

impl Window {
    pub fn new(event_loop: &ActiveEventLoop, attributes: WindowAttributes) -> Result<Self, OsError> {
        Ok(Self(Arc::new(event_loop.create_window(attributes)?)))
    }

    pub fn get_handle(&self) -> Arc<winit::window::Window> {
        self.0.clone()
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.0.inner_size()
    }

    pub fn get_scale_factor(&self) -> f64 {
        self.0.scale_factor()
    }

    pub fn request_update(&self) {
        self.0.request_redraw();
    }
}