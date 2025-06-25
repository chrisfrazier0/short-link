mod common;

use crate::common::spawn_app;
use claims::*;
use reqwest::Client;
use serde::Deserialize;
// use short_link::configuration::Configuration;
// use sqlx::{Connection, PgConnection};

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct NewLinkResponse {
  id: String,
  short: String,
  full: String,
  created_at: String,
  updated_at: String,
}

#[tokio::test]
async fn link_returns_201_for_valid_data() {
  // Arrange
  let address = spawn_app();
  let client = Client::new();
  // let config = Configuration::load("SLINK", None).expect("Failed to load configuration");
  // let conn_string = config.database.connection_string();
  // let mut conn = PgConnection::connect(&conn_string)
  //   .await
  //   .expect("Failed to connect to postgres");

  // Act
  // TODO: supply sample data for the new link body
  let body = r#"{ "url": "http://google.com" }"#;
  let response = client
    .post(format!("{}/_/link", address))
    .header("Content-Type", "application/json")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

  // Assert
  assert_eq!(response.status().as_u16(), 201);
  let response: Result<NewLinkResponse, reqwest::Error> = response.json().await;
  // TODO: better tests for link response
  assert_ok!(response);

  // let saved = sqlx::query!("SELECT id, code, url FROM links")
  //   .fetch_one(&mut conn)
  //   .await
  //   .expect("Failed to fetch saved link");

  // assert_eq!(saved.url, "http://google.com");
  // assert_eq!(saved.code.len(), 3);
}

#[tokio::test]
async fn link_returns_400_when_input_missing() {
  // Arrange
  let address = spawn_app();
  let client = Client::new();

  // Act
  let response = client
    .post(format!("{}/_/link", address))
    .header("Content-Type", "application/json")
    .send()
    .await
    .expect("Failed to execute request");

  // Assert
  assert_eq!(response.status().as_u16(), 400);
  // TODO: better tests for link 400 message
}
