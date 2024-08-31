use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Model {
    #[sqlx(rename = "meme_id")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub object_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateForm {
    #[validate(required)]
    pub name: Option<String>,
    pub description: Option<String>,
    #[validate(required)]
    pub object_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateForm {
    pub name: Option<String>,
    pub description: Option<String>,
}
