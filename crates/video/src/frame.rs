use painter::{Brush, Painter};
use sculptor::{Chisel, Sculptor};

pub struct Frame<'h, 'r> {
    pub(crate) frame: gpu::Frame<'h>,
    pub painter: &'r Painter,
    pub sculptor: &'r Sculptor,
}

impl<'q, 'r> Frame<'q, 'r> {
    pub fn draw_2d<'f>(&'f mut self) -> Brush<'q, 'f, 'r> { 
        Brush::create(painter::RenderType, &mut self.frame, self.painter)
    }

    pub fn draw_3d<'f>(&'f mut self, render_type: sculptor::RenderType) -> Chisel<'q, 'f, 'r> {
        Chisel::create(render_type, &mut self.frame, self.sculptor)
    }
}
