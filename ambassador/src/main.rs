use anyhow::Result;
use hardware::Client;

mod hardware;

#[tokio::main]
async fn main() -> Result<()> {
    Client::connect().await?;
    Ok(())
}
