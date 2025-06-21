pub extern crate gpu;
pub extern crate painter;
pub extern crate sculptor;

use gpu::surface;
use math::ext::ext2u;

pub use frame::Frame;
use math::color::Rgba;
use painter::Painter;
use sculptor::Sculptor;

mod frame;

pub struct Video<'w> {
    pub handle: gpu::Handle,
    surface: gpu::Surface<'w>,
    pub painter: Painter,
    pub sculptor: Sculptor,
    clear_color: Rgba<f64>,
}

pub struct Options {
    pub resolution: ext2u,
    pub clear_color: Rgba<f64>,
    pub painter: painter::Options,
    pub sculptor: sculptor::Options,
}

impl<'w> Video<'w> {
    pub fn create(target: impl Into<surface::Target<'w>>, options: Options) -> Self {
        let (handle, surface) = gpu::create(target, options.resolution);
        let mut painter = Painter::create(&handle, options.painter);
        painter.set_resolution(&handle, options.resolution);
        let sculptor = Sculptor::create(&handle, options.sculptor);

        Self {
            handle,
            surface,
            painter: painter,
            sculptor: sculptor,
            clear_color: options.clear_color,
        }
    }

    pub fn set_resolution(&mut self, resolution: impl Into<ext2u>) {
        let resolution = resolution.into();
        self.surface
            .set_resolution(&self.handle, resolution);
        self.painter
            .set_resolution(&self.handle, resolution);
    }

    pub fn resolution(&self) -> ext2u {
        self.surface.resolution()
    }

    pub fn start_drawing(&self) -> Frame<'_, '_> {
        Frame {
            frame: gpu::Frame::new(
                &self.handle,
                &self.surface,
                gpu::frame::Options {
                    clear_color: Some(self.clear_color),
                },
            ),
            painter: &self.painter,
            sculptor: &self.sculptor,
        }
    }
}
