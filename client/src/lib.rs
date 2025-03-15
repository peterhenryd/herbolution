#![feature(duration_constants)]
#![feature(const_trait_impl)]
#![feature(box_patterns)]

use winit::error::EventLoopError;
use winit::event_loop::EventLoop;
use crate::app::handler::Handler;

pub mod app;
pub mod session;

pub fn start() -> Result<(), EventLoopError> {
    EventLoop::new()?.run_app(&mut Handler::default())
}