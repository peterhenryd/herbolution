pub extern crate audio;
pub extern crate video;

pub mod input;

use crate::input::Input;
use audio::Audio;
use video::gpu::surface;
use video::Video;

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