use anyhow::{Context, Result, anyhow};
use config::Config;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
  pub server: ServerConfig,
  pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
  pub host: String,
  pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
  pub host: String,
  pub port: u16,
  pub username: String,
  pub password: SecretString,
  pub name: String,
}

impl DatabaseConfig {
  pub fn connection_string(&self) -> SecretString {
    SecretString::from(format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username,
      self.password.expose_secret(),
      self.host,
      self.port,
      self.name,
    ))
  }
}

impl Configuration {
  pub fn load(prefix: &str, file: Option<&str>) -> Result<Self> {
    let file = match file {
      Some(file) => file,
      None => {
        let cwd = std::env::current_dir()?;
        &cwd
          .join("config/config.yml")
          .to_str()
          .ok_or(anyhow!("Failed to build config path"))?
          .to_string()
      }
    };
    let config = Config::builder()
      .add_source(config::File::with_name(file))
      .add_source(
        config::Environment::with_prefix(prefix)
          .separator("_")
          .prefix_separator("_"),
      )
      .build()
      .context("Failed to build configuration")?
      .try_deserialize()
      .context("Failed to deserialize configuration")?;
    Ok(config)
  }
}
