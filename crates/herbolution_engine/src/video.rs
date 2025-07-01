use gpu::surface;
use math::color::Rgba;
use math::size::size2u;

use crate::painter::brush::Brush;
use crate::painter::Painter;
use crate::sculptor::{Chisel, Sculptor};
use crate::{painter, sculptor};

pub struct Video<'w> {
    pub handle: gpu::Handle,
    surface: gpu::Surface<'w>,
    pub painter: Painter,
    pub sculptor: Sculptor,
    clear_color: Rgba<f64>,
}

pub struct Options {
    pub resolution: size2u,
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

    pub fn set_resolution(&mut self, resolution: impl Into<size2u>) {
        let resolution = resolution.into();
        self.surface.set_resolution(&self.handle, resolution);
        self.painter.set_resolution(&self.handle, resolution);
    }

    pub fn resolution(&self) -> size2u {
        self.surface.resolution()
    }

    pub fn create_frame(&self) -> Frame<'_, '_> {
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

pub struct Frame<'h, 'a> {
    pub(crate) frame: gpu::Frame<'h>,
    pub painter: &'a Painter,
    pub sculptor: &'a Sculptor,
}

impl<'h, 'a> Frame<'h, 'a> {
    pub fn draw_2d<'f>(&'f mut self) -> Brush<'h, 'f, 'a> {
        Brush::create(painter::RenderType, &mut self.frame, self.painter)
    }

    pub fn draw_3d<'f>(&'f mut self, render_type: sculptor::RenderType) -> Chisel<'h, 'f, 'a> {
        Chisel::create(render_type, &mut self.frame, self.sculptor)
    }

    pub fn advance_pass(&mut self, clear_color: Option<Rgba<f64>>, no_depth: bool) {
        self.frame.advance_pass(clear_color, no_depth);
    }

    pub fn clear_color(&mut self, color: Rgba<f64>) {
        self.frame.advance_pass(Some(color), false);
    }
}
