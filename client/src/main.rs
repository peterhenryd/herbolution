#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::try_init()?;
    herbolution_client::start()?;

    Ok(())
}