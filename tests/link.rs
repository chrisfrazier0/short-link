mod common;

use crate::common::prepare;
use claims::*;
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
  let test = prepare().await;

  // Act
  let body = r#"{ "url": "http://google.com" }"#;
  let response = test
    .client
    .post(format!("{}/_/link", test.address))
    .header("Content-Type", "application/json")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

  // Assert
  assert_eq!(response.status().as_u16(), 201);
  let response: Result<NewLinkResponse, reqwest::Error> = response.json().await;
  assert_ok!(response);

  let saved = sqlx::query!("SELECT id, code, url FROM links")
    .fetch_one(&test.db_pool)
    .await
    .expect("Failed to fetch saved link");

  assert_eq!(saved.url, "http://google.com");
  assert_eq!(saved.code.len(), 3);
}

#[tokio::test]
async fn link_returns_400_when_input_missing() {
  // Arrange
  let test = prepare().await;

  // Act
  let response = test
    .client
    .post(format!("{}/_/link", test.address))
    .header("Content-Type", "application/json")
    .send()
    .await
    .expect("Failed to execute request");

  // Assert
  assert_eq!(response.status().as_u16(), 400);
  // TODO: better tests for link 400 message
}
