use anyhow::{Context, Result};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").context("Failed to bind address")?;
  short_link::run(listener)?
    .await
    .context("Failed to run server")
}
