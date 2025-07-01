extern crate herbolution_gpu as gpu;
extern crate herbolution_math as math;

pub mod audio;
pub mod input;
pub mod painter;
pub mod sculptor;
pub mod ui;
pub mod video;

use gpu::surface;
use video::Video;

use crate::audio::Audio;
use crate::input::Input;

pub struct Engine<'w> {
    pub audio: Audio,
    pub video: Video<'w>,
    pub input: Input,
}

pub struct Options {
    pub video: video::Options,
}

impl<'w> Engine<'w> {
    pub fn create(target: impl Into<surface::Target<'w>>, options: Options) -> Self {
        let audio = Audio::new();
        let video = Video::create(target, options.video);
        let input = Input::default();

        Self { audio, video, input }
    }
}
