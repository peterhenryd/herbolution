use clap::builder::PathBufValueParser;
use clap::{Arg, Command};
use herbolution_client::Options;
use std::env::home_dir;
use std::path::PathBuf;
use winit::error::EventLoopError;

fn command() -> Command {
    Command::new("herbolution")
        .arg(Arg::new("data_dir")
            .value_parser(PathBufValueParser::new())
            .required(false)
        )
}

#[tokio::main]
async fn main() -> Result<(), EventLoopError> {
    let matches = command().get_matches();
    let data_dir = matches.get_one::<PathBuf>("data_dir").cloned()
        .unwrap_or(home_dir().unwrap_or(PathBuf::from(".")).join(".herbolution"));

    if let Err(e) = tracing_subscriber::fmt::try_init() {
        eprintln!("Failed to initialize tracing subscriber: {}", e);
    }

    herbolution_client::start(Options { data_dir })
}