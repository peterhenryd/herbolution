#![feature(duration_constants)]
#![feature(const_trait_impl)]
extern crate herbolution_math as math;

pub mod engine;
pub mod game;
pub mod handler;
pub mod listener;
pub mod ui;
pub mod world;

pub fn start(options: Options) -> Result<(), winit::error::EventLoopError> {
    use lazy_winit::EventLoopExt;

    winit::event_loop::EventLoop::new()?
        .run_lazy_app::<handler::Handler>(options)
}

pub struct Options {}