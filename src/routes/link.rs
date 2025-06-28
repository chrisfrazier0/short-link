use actix_web::{HttpResponse, Responder, web};
use anyhow::Result;
use chrono::Utc;
use rand::Rng;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NewLinkPayload {
  pub url: String,
}

#[tracing::instrument(
  name = "Adding a new link",
  skip(db_pool, data),
  fields(url = %data.url),
)]
pub async fn create_short_link(
  db_pool: web::Data<PgPool>,
  data: web::Json<NewLinkPayload>,
) -> impl Responder {
  match insert_link(&db_pool, &data).await {
    Ok(_) => HttpResponse::Created().body(format!(
      r#"{{
  "id": "",
  "short": "",
  "full": "{}",
  "created_at": "",
  "updated_at": ""
}}"#,
      data.url,
    )),
    Err(e) => {
      tracing::error!("Failed to execute query: {:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[tracing::instrument(name = "Saving new link to the database", skip(db_pool, data))]
async fn insert_link(db_pool: &PgPool, data: &NewLinkPayload) -> Result<()> {
  let id = Uuid::new_v4();
  let now = Utc::now();

  let chars = b"abcdefghijklmnopqrstuvwxyz0123456789";
  let mut rng = rand::rng();
  let code: String = (0..3)
    .map(|_| chars[rng.random_range(0..chars.len())] as char)
    .collect();

  sqlx::query!(
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
  .execute(db_pool)
  .await?;
  Ok(())
}
