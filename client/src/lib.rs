#![feature(duration_constants)]
#![feature(const_trait_impl)]
#![feature(box_patterns)]
#![feature(random)]

use std::path::PathBuf;
use winit::error::EventLoopError;
use winit::event_loop::EventLoop;
use crate::app::handler::Handler;

pub mod app;
pub mod session;

pub struct Options {
    pub data_dir: PathBuf,
}

pub fn start(options: Options) -> Result<(), EventLoopError> {
    EventLoop::new()?.run_app(&mut Handler::new(options))
}