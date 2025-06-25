mod common;

use crate::common::spawn_app;
use reqwest::Client;
use serde::Deserialize;

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
  assert_eq!(201, response.status().as_u16());
  let response: Result<NewLinkResponse, reqwest::Error> = response.json().await;
  // TODO: better tests for link response
  assert!(!response.is_ok());
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
  assert_eq!(400, response.status().as_u16());
  // TODO: better tests for link 400 message
}
