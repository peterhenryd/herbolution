#![feature(iter_next_chunk)]
#![feature(random)]

extern crate herbolution_lib as lib;
extern crate herbolution_server as server;

pub mod app;
pub mod input;
pub mod menu;
pub mod session;
pub mod ui;
pub mod video;
pub mod world;

pub fn run() -> Result<(), winit::error::EventLoopError> {
    app::App::new(clap::Parser::parse()).run()
}
