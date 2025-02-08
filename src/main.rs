extern crate herbolution_lib as lib;

mod command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = tracing_subscriber::fmt::try_init() {
        eprintln!("Failed to initialize tracing: {:?}", e);
    }

    let command: command::Command = clap::Parser::parse();

    lib::start(command.into())?;

    Ok(())
}