mod common;

use crate::common::spawn_app;
use reqwest::Client;

#[tokio::test]
async fn health_check_works() {
  // Arrange
  let address = spawn_app();
  let client = Client::new();

  // Act
  let response = client
    .get(format!("{}/_/health_check", address))
    .send()
    .await
    .expect("Failed to execute request");

  // Assert
  assert_eq!(200, response.status().as_u16());
  assert_eq!(Some(0), response.content_length());
}
