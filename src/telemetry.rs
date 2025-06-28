use anyhow::{Context, Result};
use dotenv::dotenv;
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, fmt::MakeWriter, layer::SubscriberExt};

#[derive(Debug, Clone)]
pub struct Telemetry {
  name: String,
  env_filter: String,
}

impl Telemetry {
  pub fn new(name: &str, env_filter: &str) -> Self {
    Self {
      name: name.into(),
      env_filter: env_filter.into(),
    }
  }

  pub fn init<Sink>(self, sink: Sink) -> Result<()>
  where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
  {
    dotenv().ok();
    LogTracer::init().context("Failed to set logger")?;
    set_global_default(self.subscriber(sink)).context("Failed to set subscriber")?;
    Ok(())
  }

  fn subscriber<Sink>(&self, sink: Sink) -> impl Subscriber
  where
    Sink: for<'a> MakeWriter<'a> + 'static,
  {
    let env_filter =
      EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(self.env_filter.clone()));
    let formatting_layer = BunyanFormattingLayer::new(self.name.clone(), sink);
    Registry::default()
      .with(env_filter)
      .with(JsonStorageLayer)
      .with(formatting_layer)
  }
}
