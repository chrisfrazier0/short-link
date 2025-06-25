use anyhow::{Context, Result};
use config::Config;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Configuration {
  pub port: Option<u16>,

  #[serde(default)]
  pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DatabaseConfig {
  pub host: Option<String>,
  pub port: Option<u16>,
  pub username: Option<String>,
  pub password: Option<String>,
  pub name: Option<String>,
}

impl Configuration {
  pub fn load(prefix: &str, file: Option<&str>) -> Result<Self> {
    let builder = match file {
      Some(file) => Config::builder().add_source(config::File::with_name(file)),
      None => Config::builder(),
    };
    let built = builder
      .add_source(
        config::Environment::with_prefix(prefix)
          .separator("_")
          .prefix_separator("_"),
      )
      .build()
      .context("Failed to build configuration")?;
    let mut config: Configuration = built
      .try_deserialize()
      .context("Failed to deserialize configuration")?;
    let database = &mut config.database;

    config.port.get_or_insert(8080);

    database.host.get_or_insert("127.0.0.1".into());
    database.port.get_or_insert(5432);
    database.username.get_or_insert("app".into());
    database.password.get_or_insert("secret".into());
    database.name.get_or_insert("shortlinks".into());

    Ok(config)
  }
}

impl DatabaseConfig {
  pub fn connection_string(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username.as_ref().unwrap(),
      self.password.as_ref().unwrap(),
      self.host.as_ref().unwrap(),
      self.port.as_ref().unwrap(),
      self.name.as_ref().unwrap(),
    )
  }
}
