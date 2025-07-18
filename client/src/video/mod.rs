use std::path::PathBuf;

use lib::color::Rgba;
use lib::size::size2u;
use wgpu::SurfaceTarget;

use crate::video::resource::SampleCount;
use crate::video::ui::brush::Brush;
use crate::video::ui::Painter;
use crate::video::world::chisel::Chisel;
use crate::video::world::Sculptor;

pub mod camera;
pub mod frame;
pub mod fx;
pub mod gpu;
pub mod resource;
pub mod ui;
pub mod world;

#[derive(Debug)]
pub struct Video<'w> {
    pub handle: gpu::Handle,
    pub surface: gpu::Surface<'w>,
    pub painter: Painter,
    pub sculptor: Sculptor,
    clear_color: Rgba<f64>,
}

#[derive(Debug)]
pub struct Options {
    pub resolution: size2u,
    pub clear_color: Rgba<f64>,
    pub sample_count: SampleCount,
    pub asset_path: PathBuf,
    pub vsync: bool,
}

impl<'w> Video<'w> {
    pub fn create(target: impl Into<SurfaceTarget<'w>>, options: Options) -> Self {
        let (handle, surface) = gpu::create(target, options.resolution, options.sample_count, options.vsync);
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

    pub fn create_frame(&self) -> Frame<'_> {
        Frame {
            frame: frame::Frame::new(&self.handle, &self.surface, Some(self.clear_color)),
            painter: &self.painter,
            sculptor: &self.sculptor,
        }
    }

    pub fn set_clear_color(&mut self, color: Rgba<f64>) {
        self.clear_color = color;
    }
}

#[derive(Debug)]
pub struct Frame<'a> {
    pub(crate) frame: frame::Frame<'a>,
    pub painter: &'a Painter,
    pub sculptor: &'a Sculptor,
}

impl<'a> Frame<'a> {
    #[tracing::instrument(skip_all)]
    pub fn draw_2d<'f>(&'f mut self) -> Brush<'f, 'a> {
        Brush::create(ui::RenderType, &mut self.frame, self.painter)
    }

    #[tracing::instrument(skip(self))]
    pub fn draw_3d<'f>(&'f mut self, render_type: world::RenderType) -> Chisel<'f, 'a> {
        Chisel::create(render_type, &mut self.frame, self.sculptor)
    }

    #[tracing::instrument(skip_all)]
    pub fn advance_pass(&mut self, clear_color: Option<Rgba<f64>>, no_depth: bool) {
        self.frame.advance_pass(clear_color, no_depth);
    }

    pub fn clear_color(&mut self, color: Rgba<f64>) {
        self.frame.advance_pass(Some(color), false);
    }
}
