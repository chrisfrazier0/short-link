use anyhow::{Context, Result};
use dotenv::dotenv;
use short_link::{configuration::Configuration, startup::start};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();

  let config = Configuration::load("SLINK", None)?;
  let address = format!("127.0.0.1:{}", config.port.unwrap());
  let listener = TcpListener::bind(address).context("Failed to bind address")?;

  let conn_string = config.database.connection_string();
  let db_pool = PgPool::connect(&conn_string)
    .await
    .context("Failed to connect to database")?;

  start(listener, db_pool)?
    .await
    .context("Failed to start server")
}
