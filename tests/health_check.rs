mod common;

use crate::common::prepare;

#[tokio::test]
async fn health_check_works() {
  // Arrange
  let test = prepare().await;

  // Act
  let response = test
    .client
    .get(format!("{}/_/health_check", test.address))
    .send()
    .await
    .expect("Failed to execute request");

  // Assert
  assert_eq!(200, response.status().as_u16());
  assert_eq!(Some(0), response.content_length());
}
