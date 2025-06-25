use anyhow::{Context, Result};
use dotenv::dotenv;
use short_link::{configuration::Configuration, startup::start};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();

  let config = Configuration::load("SLINK", None)?;
  let address = format!("127.0.0.1:{}", config.port.unwrap());
  let listener = TcpListener::bind(address).context("Failed to bind address")?;

  println!("{}", config.port.unwrap());
  start(listener)?.await.context("Failed to start server")
}
