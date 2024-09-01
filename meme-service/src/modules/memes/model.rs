use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{common::errors::AppError, utils::s3signer::S3Signer};

use super::BUCKET_NAME;

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

#[derive(Serialize, Deserialize)]
pub struct PubModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub s3_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Model {
    pub async fn to_pub(self, signer: &S3Signer) -> Result<PubModel, AppError> {
        let m = PubModel {
            id: self.id,
            name: self.name,
            description: self.description,
            s3_url: signer.sign_get(BUCKET_NAME, self.s3_path, 3600).await?,
            created_at: self.created_at,
            updated_at: self.updated_at,
        };

        Ok(m)
    }
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
    pub meta: MpJson<CreateForm>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateForm {
    pub name: Option<String>,
    pub description: Option<String>,
}
