use rmcp::ServiceExt;
use tracing_subscriber::{self, EnvFilter};

mod cerebro;
use crate::cerebro::Cerebro;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Cerebro MCP server...");

    let service = Cerebro::new()
        .serve(rmcp::transport::stdio())
        .await
        .inspect_err(|e| tracing::error!("server error: {:?}", e))?;

    tracing::info!("Cerebro MCP server running. Waiting for requests...");

    service.waiting().await?;

    Ok(())
}
