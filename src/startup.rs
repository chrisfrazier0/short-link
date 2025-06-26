use crate::routes::{create_short_link, health_check};
use actix_web::{App, HttpServer, dev::Server, web};
use anyhow::{Context, Result};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn start(listener: TcpListener, db_pool: PgPool) -> Result<Server> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
    App::new()
      .app_data(db_pool.clone())
      .route("/_/health_check", web::get().to(health_check))
      .route("/_/link", web::post().to(create_short_link))
  })
  .listen(listener)
  .context("Failed to listen on provided listener")?
  .run();
  Ok(server)
}
