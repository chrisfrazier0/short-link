use anyhow::{Context, Result};
use secrecy::ExposeSecret;
use short_link::{configuration::Configuration, startup::start, telemetry::Telemetry};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
  Telemetry::new("short-link", "info").init(std::io::stdout)?;

  let config = Configuration::load("SLINK", None)?;
  let address = format!("127.0.0.1:{}", config.port.unwrap());
  let listener = TcpListener::bind(address).context("Failed to bind address")?;

  let conn_string = config.database.connection_string();
  let db_pool = PgPool::connect(conn_string.expose_secret())
    .await
    .context("Failed to connect to database")?;

  start(listener, db_pool)?
    .await
    .context("Failed to start server")
}
