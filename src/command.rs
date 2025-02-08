use clap::Parser;
use lib::Options;

#[derive(Parser)]
pub struct Command {}

impl Into<Options> for Command {
    fn into(self) -> Options {
        Options {}
    }
}