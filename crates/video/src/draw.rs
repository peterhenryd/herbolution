pub struct Drawing<'q, 'r> {
    pub(super) frame: gpu::Frame<'q>,
    pub(super) r2d: &'r video_2d::Renderer,
    pub(super) r3d: &'r video_3d::Renderer,
}

impl<'q, 'r> Drawing<'q, 'r> {
    pub fn begin_2d<'f>(&'f mut self) -> video_2d::Drawing<'q, 'f, 'r> {
        video_2d::Drawing::create(v2d::RenderType, &mut self.frame, self.r2d)
    }
    
    pub fn begin_3d<'f>(&'f mut self, render_type: v3d::RenderType) -> v3d::Drawing<'q, 'f, 'r> {
        v3d::Drawing::create(render_type, &mut self.frame, self.r3d)
    }
}