pub extern crate gpu;
pub extern crate video_2d as v2d;
pub extern crate video_3d as v3d;

use gpu::{frame, surface};
use math::size::Size2;

mod draw;

use math::color::Rgba;

pub type Drawing<'q, 'r> = draw::Drawing<'q, 'r>;

pub struct Video<'w> {
    pub handle: gpu::Handle,
    surface: gpu::Surface<'w>,
    pub r2d: v2d::Renderer,
    pub r3d: v3d::Renderer,
    clear_color: Rgba<f64>,
}

pub struct Options {
    pub resolution: Size2<u32>,
    pub clear_color: Rgba<f64>,
    pub r2d: v2d::Options,
    pub r3d: v3d::Options,
}

impl<'w> Video<'w> {
    pub fn create(target: impl Into<surface::Target<'w>>, options: Options) -> Self {
        let (handle, surface) = gpu::create(target, options.resolution);
        let mut r2d = v2d::Renderer::create(&handle, options.r2d);
        r2d.set_resolution(&handle, options.resolution);
        let r3d = v3d::Renderer::create(&handle, options.r3d);

        Self {
            handle,
            surface,
            r2d,
            r3d,
            clear_color: options.clear_color,
        }
    }

    pub fn set_resolution(&mut self, resolution: impl Into<Size2<u32>>) {
        let resolution = resolution.into();
        self.surface
            .set_resolution(&self.handle, resolution);
        self.r2d
            .set_resolution(&self.handle, resolution);
    }

    pub fn resolution(&self) -> Size2<u32> {
        self.surface.resolution()
    }

    pub fn start_drawing(&self) -> Drawing<'_, '_> {
        Drawing {
            handle: &self.handle,
            frame: gpu::Frame::new(
                &self.handle,
                &self.surface,
                frame::Options {
                    clear_color: Some(self.clear_color),
                },
            )
            .into_owned(),
            r2d: &self.r2d,
            r3d: &self.r3d,
        }
    }
}
