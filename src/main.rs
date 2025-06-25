use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use anyhow::{Context, Result};

async fn health_check() -> impl Responder {
  HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<()> {
  HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
    .bind("127.0.0.1:8080")
    .context("Failed to bind address")?
    .run()
    .await
    .context("Failed to run server")
}
