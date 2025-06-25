use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
  HttpResponse::Ok()
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct NewLinkPayload {
  url: String,
}

async fn create_short_link(data: web::Json<NewLinkPayload>) -> impl Responder {
  HttpResponse::Created().body(format!("link url: {}", data.url))
}

pub fn run(listener: TcpListener) -> Result<Server> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/_/health_check", web::get().to(health_check))
      .route("/_/link", web::post().to(create_short_link))
  })
  .listen(listener)
  .context("Failed to listen on provided listener")?
  .run();
  Ok(server)
}
