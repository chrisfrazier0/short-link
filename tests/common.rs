use reqwest::Client;
use short_link::{
  configuration::{Configuration, DatabaseConfig},
  startup::start,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestFixtures {
  pub client: Client,
  pub address: String,

  #[allow(dead_code)]
  pub db_pool: PgPool,
}

pub async fn prepare() -> TestFixtures {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let mut config = Configuration::load("SLINK", None).expect("Failed to load configuration");
  config.database.name = Some(Uuid::new_v4().to_string());
  let db_pool = configure_database(&config.database).await;

  let server = start(listener, db_pool.clone()).expect("Failed to create server");
  tokio::spawn(server);

  TestFixtures {
    client: Client::new(),
    address,
    db_pool,
  }
}

pub async fn configure_database(config: &DatabaseConfig) -> PgPool {
  let maintenance = DatabaseConfig {
    name: Some("postgres".to_string()),
    ..config.clone()
  };

  let mut conn = PgConnection::connect(&maintenance.connection_string())
    .await
    .expect("Failed to connect to postgres");

  let name = config.name.clone().unwrap();
  conn
    .execute(format!("CREATE DATABASE \"{}\";", name).as_str())
    .await
    .expect("Failed to create database");

  let db_pool = PgPool::connect(&config.connection_string())
    .await
    .expect("Failed to create connection pool");

  sqlx::migrate!("./migrations")
    .run(&db_pool)
    .await
    .expect("Failed to run migrations");

  db_pool
}
