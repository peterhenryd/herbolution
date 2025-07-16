extern crate herbolution_client as client;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    init_tracing();

    if let Some(home_dir) = std::env::home_dir()
        && home_dir.join(".herbolution").exists()
    {
        tracing::warn!("The .herbolution directory in your home folder is deprecated and no longer used. Please consider removing it.");
    }

    client::run()
}

fn init_tracing() {
    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env());

    #[cfg(feature = "tracing")]
    let registry = registry.with(tracing_tracy::TracyLayer::default());

    if let Err(e) = registry.try_init() {
        eprintln!("Failed to initialize tracing: {}", e);
    }
}
