extern crate herbolution_client as client;

use clap::builder::PathBufValueParser;
use clap::{Arg, Command};
use client::app::App;
use client::trace::trace_init;
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    let _guard = trace_init();

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
