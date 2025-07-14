extern crate herbolution_client as client;

use clap::builder::PathBufValueParser;
use clap::{Arg, Command};
use client::app::App;
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    if let Err(e) = tracing_subscriber::fmt::try_init() {
        eprintln!("Failed to initialize tracing subscriber: {}", e);
    }

    let matches = command().get_matches();
    let root_dir = matches.get_one("root").cloned();

    App::new(root_dir).run()
}

fn command() -> Command {
    let root_arg = Arg::new("root")
        .long("root")
        .value_parser(PathBufValueParser::new())
        .required(false);

    Command::new("herbolution").arg(root_arg)
}
