use gpu::{FrameOptions, SampleCount, SurfaceTarget};
use math::color::Rgba;
use math::size::size2u;
use std::path::PathBuf;

use crate::painter::brush::Brush;
use crate::painter::Painter;
use crate::sculptor::chisel::Chisel;
use crate::sculptor::Sculptor;
use crate::{painter, sculptor};

pub struct Video<'w> {
    pub handle: gpu::Handle,
    pub surface: gpu::Surface<'w>,
    pub painter: Painter,
    pub sculptor: Sculptor,
    clear_color: Rgba<f64>,
}

pub struct Options {
    pub resolution: size2u,
    pub clear_color: Rgba<f64>,
    pub painter: painter::Options,
    pub sculptor: sculptor::Options,
    pub sample_count: SampleCount,
    pub asset_path: PathBuf,
}

impl<'w> Video<'w> {
    pub fn create(target: impl Into<SurfaceTarget<'w>>, options: Options) -> Self {
        let (handle, surface) = gpu::create(target, options.resolution, options.sample_count);
        let mut painter = Painter::create(&handle, options.sample_count, &options.asset_path);
        painter.set_resolution(&handle, options.resolution);
        let sculptor = Sculptor::create(&handle, options.sample_count);

        Self {
            handle,
            surface,
            painter,
            sculptor,
            clear_color: options.clear_color,
        }
    }

    pub fn set_sample_count(&mut self, sample_count: SampleCount) {
        self.surface
            .set_sample_count(&self.handle, sample_count);
        self.painter
            .set_sample_count(&self.handle, sample_count);
        self.sculptor
            .set_sample_count(&self.handle, sample_count);
    }

    pub fn set_resolution(&mut self, resolution: impl Into<size2u>) {
        let resolution = resolution.into();
        self.surface
            .set_resolution(&self.handle, resolution);
        self.painter
            .set_resolution(&self.handle, resolution);
    }

    pub fn resolution(&self) -> size2u {
        self.surface.resolution()
    }

    pub fn create_frame(&self) -> Frame<'_, '_> {
        Frame {
            frame: gpu::Frame::new(
                &self.handle,
                &self.surface,
                FrameOptions {
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
