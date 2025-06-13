pub extern crate audio;
pub extern crate video;

pub mod input;

use crate::input::Input;
use math::size::Size2;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use audio::Audio;
use video::Video;

pub struct Engine<'w> {
    pub window: Arc<Window>,
    pub audio: Audio,
    pub video: Video<'w>,
    pub input: Input,
}

pub struct Options {
    pub video: video::Options,
}

impl<'w> Engine<'w> {
    pub fn create(window: Arc<Window>, options: Options) -> Self {
        let audio = Audio::new();
        let PhysicalSize { width, height } = window.inner_size();
        let video = Video::create(window.clone(), Size2::new(width, height), options.video);
        let input = Input::default();

        Self { window, audio, video, input }
    }
}