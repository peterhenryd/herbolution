use std::collections::VecDeque;

use crate::Texture;

pub struct Taa {
    prev_frame_buffers: VecDeque<Texture>,
}

impl Taa {}
