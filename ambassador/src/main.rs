use actix_web::{web, App, HttpServer};
use anyhow::Result;

mod hardware;

#[tokio::main]
async fn main() -> Result<()> {
    // Client::connect().await?;
    HttpServer::new(|| App::new().route("/ping", web::get().to(ping)))
        .bind(("0.0.0.0", 8089))?
        .run()
        .await?;
    Ok(())
}

async fn ping() -> &'static str {
    "pong"
}
