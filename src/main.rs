use anyhow::Result;
use system_manager::hardware::monitor::WorkerManager;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    info!("start");
    WorkerManager::start_server()?;

    std::future::pending().await
}
