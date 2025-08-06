#![feature(iter_next_chunk)]
#![feature(random)]
extern crate herbolution_lib as lib;
extern crate herbolution_server as server;
extern crate kiln_prims as prims;

pub mod app;
pub mod menu;
pub mod session;
pub mod ui;

pub fn run() -> Result<(), winit::error::EventLoopError> {
    winit::event_loop::EventLoop::new()?.run_app(&mut app::handler::AppHandler::new(app::AppOptions {
        data_dir: directories::BaseDirs::new()
            .unwrap()
            .data_local_dir()
            .join("Herbolution"),
        resolution: prims::size::Size2::new(960, 600),
    }))
}
