use std::env::home_dir;
use std::path::PathBuf;

use clap::builder::PathBufValueParser;
use clap::{Arg, Command};
use herbolution_client::app;
use herbolution_client::app::App;
use winit::error::EventLoopError;

fn command() -> Command {
    let root_arg = Arg::new("root").long("root").value_parser(PathBufValueParser::new()).required(false);

    Command::new("herbolution").arg(root_arg)
}

fn main() -> Result<(), EventLoopError> {
    let matches = command().get_matches();
    let root_path = matches
        .get_one::<PathBuf>("root")
        .cloned()
        .unwrap_or(home_dir().unwrap_or(PathBuf::from(".")).join(".herbolution"));

    if let Err(e) = tracing_subscriber::fmt::try_init() {
        eprintln!("Failed to initialize tracing subscriber: {}", e);
    }

    App::new(app::Options { root_path }).run()
}
