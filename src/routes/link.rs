use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NewLinkPayload {
  pub url: String,
}

pub async fn create_short_link(data: web::Json<NewLinkPayload>) -> impl Responder {
  HttpResponse::Created().body(format!(
    r#"{{
      "id": "",
      "short": "",
      "full": "{}",
      "created_at": "",
      "updated_at": ""
    }}"#,
    data.url,
  ))
}
