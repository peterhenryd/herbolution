use std::path::Path;
use std::{fs, io};

use clap::builder::PathBufValueParser;
use clap::{Arg, Command};
use herbolution_client::app::{App, Options};
use herbolution_lib::root_dir;
use include_dir::{Dir, include_dir};
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    if let Err(e) = tracing_subscriber::fmt::try_init() {
        eprintln!("Failed to initialize tracing subscriber: {}", e);
    }

    copy_assets(&root_dir().join("assets")).unwrap();

    App::new(options()).run()
}

fn options() -> Options {
    let matches = command().get_matches();
    Options {
        root_path: matches.get_one("root").cloned().unwrap_or(root_dir()),
    }
}

fn command() -> Command {
    let root_arg = Arg::new("root")
        .long("root")
        .value_parser(PathBufValueParser::new())
        .required(false);

    Command::new("herbolution").arg(root_arg)
}

fn copy_assets(base_path: &Path) -> io::Result<()> {
    const DIR: Dir<'_> = include_dir!("assets");

    if !base_path.exists() {
        fs::create_dir_all(base_path)?;
    }

    let mut entries = DIR.entries().to_vec();
    while let Some(entry) = entries.pop() {
        let path = base_path.join(entry.path());

        if let Some(file) = entry.as_file() {
            if !path.exists() {
                fs::write(&path, file.contents())?;
            }
        } else if let Some(dir) = entry.as_dir() {
            if !path.exists() {
                fs::create_dir(&path)?;
            }

            entries.extend_from_slice(dir.entries());
        }
    }

    fs::write(
        base_path.join("README"),
        "Please do not edit the contents of this directory manually; it is overwritten frequently without warning.",
    )?;

    Ok(())
}
