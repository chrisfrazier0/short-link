use crate::routes::{create_short_link, health_check};
use actix_web::{App, HttpServer, dev::Server, web};
use anyhow::{Context, Result};
use std::net::TcpListener;

pub fn start(listener: TcpListener) -> Result<Server> {
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
