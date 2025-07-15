#![feature(duration_constants)]
#![feature(random)]
extern crate herbolution_lib as lib;
extern crate herbolution_server as server;

pub mod app;
pub mod input;
pub mod menu;
pub mod session;
pub mod trace;
mod ui;
pub mod video;
pub mod world;

pub fn run(root_path: Option<std::path::PathBuf>) -> Result<(), winit::error::EventLoopError> {
    app::App::new(root_path).run()
}
