use rmcp::ServiceExt;

mod cerebro;
use crate::cerebro::Cerebro;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting Cerebro MCP server...");

    let service = Cerebro::new().serve(rmcp::transport::stdio()).await?;

    println!("Cerebro MCP server running. Waiting for requests...");

    service.waiting().await?;

    Ok(())
}
