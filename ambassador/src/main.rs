use actix_web::{web, App, HttpServer};
use anyhow::Result;

mod hardware;
mod myself;
mod sys_info;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/info", web::get().to(sys_info::get_sys_info))
            .route("/cmd/uninstall", web::get().to(myself::uninstall))
            .route("/ping", web::get().to(ping))
    })
    .bind(("0.0.0.0", 8089))?
    .run()
    .await?;
    Ok(())
}

async fn ping() -> &'static str {
    "pong"
}
