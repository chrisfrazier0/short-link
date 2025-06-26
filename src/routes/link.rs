use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use rand::Rng;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NewLinkPayload {
  pub url: String,
}

pub async fn create_short_link(
  db_pool: web::Data<PgPool>,
  data: web::Json<NewLinkPayload>,
) -> impl Responder {
  let id = Uuid::new_v4();
  let now = Utc::now();

  let chars = b"abcdefghijklmnopqrstuvwxyz0123456789";
  let mut rng = rand::rng();
  let code: String = (0..3)
    .map(|_| chars[rng.random_range(0..chars.len())] as char)
    .collect();

  match sqlx::query!(
    r#"
    INSERT INTO links (id, code, url, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5)
    "#,
    id,
    code,
    data.url,
    now,
    now,
  )
  .execute(db_pool.get_ref())
  .await
  {
    Ok(_) => HttpResponse::Created().body(format!(
      r#"{{
        "id": "{id}",
        "short": "{code}",
        "full": "{}",
        "created_at": "{now}",
        "updated_at": "{now}"
      }}"#,
      data.url,
    )),
    Err(e) => {
      eprintln!("Failed to execute query: {}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
