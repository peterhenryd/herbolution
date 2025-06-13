use math::size::Size2;
use video::{SurfaceTarget, Video};

pub struct Engine<'w> {
    video: Video<'w>,
}

impl<'w> Engine<'w> {
    pub fn create(target: impl Into<SurfaceTarget<'w>>, resolution: impl Into<Size2<u32>>) -> Self {
        let video = Video::create(target, resolution);

        Self { video }
    }
}
