use lazy_winit::EventLoopExt;
use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

mod app;
mod error;
mod menu;
mod ui;
mod game;
mod world;

fn main() -> Result<(), EventLoopError> {
    EventLoop::new()?.run_lazy_app::<app::App>(())
}
