use actix_multipart::form::{tempfile::TempFile, text::Text as MpText, MultipartForm};
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

#[derive(Deserialize, Validate)]
pub struct CreateForm {
    #[validate(required)]
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Validate, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub form: MpText<CreateForm>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateForm {
    pub name: Option<String>,
    pub description: Option<String>,
}
