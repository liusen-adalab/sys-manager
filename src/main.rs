use actix_web::{App, HttpServer};
use anyhow::Result;
use system_manager::controller;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("start");

    HttpServer::new(move || App::new().configure(controller::http::config_endpoints))
        .bind("0.0.0.0:8999")?
        .run()
        .await?;

    Ok(())
}
