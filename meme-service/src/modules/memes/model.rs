use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Model {
    #[sqlx(rename = "meme_id")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub s3_path: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
