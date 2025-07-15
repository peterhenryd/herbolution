use tracing_subscriber::prelude::*;

pub fn trace_init() {
    #[cfg(feature = "tracing")]
    tracing_subscriber::registry()
        .with(tracing_tracy::TracyLayer::default())
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    #[cfg(not(feature = "tracing"))]
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
}
