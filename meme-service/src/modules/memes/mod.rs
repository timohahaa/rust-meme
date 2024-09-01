pub mod model;
mod queries;

use actix_multipart::form::tempfile;
use futures::TryStreamExt;
use minio::s3::{args::UploadObjectArgs, client::Client};
use model::{CreateForm, Model, UpdateForm};
use queries::{
    CREATE_MEME_QUERY, DELETE_MEME_QUERY, GET_MEME_QUERY, LIST_MEMES_QUERY, MARK_AS_DONE_QUERY,
    UPDATE_MEME_QUERY,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::common::errors;

const BUCKET_NAME: &str = "memes";

#[derive(Clone)]
pub struct Module {
    conn: Pool<Postgres>,
    s3: Client,
}

impl Module {
    pub async fn new(conn: Pool<Postgres>, s3: Client) -> Module {
        Module { conn, s3 }
    }

    pub async fn list(&self) -> Result<Vec<Model>, errors::AppError> {
        let mut rows = sqlx::query_as(LIST_MEMES_QUERY).fetch(&self.conn);
        let mut memes = vec![];

        while let Some(meme) = rows.try_next().await? {
            memes.push(meme)
        }

        Ok(memes)
    }

    pub async fn get(&self, id: Uuid) -> Result<Model, errors::AppError> {
        match sqlx::query_as::<_, Model>(GET_MEME_QUERY)
            .bind(id)
            .fetch_one(&self.conn)
            .await
        {
            Ok(model) => Ok(model),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn create(
        &self,
        form: CreateForm,
        file: tempfile::TempFile,
    ) -> Result<Model, errors::AppError> {
        // status model - so the s3 doesnt leak
        // OUTSIDE OF TRANSACTION make a record about an object
        // if upload was succesfull - mark it as done
        let id = uuid::Uuid::new_v4();
        let s3_path = gen_s3_path(id);
        let model = sqlx::query_as::<_, Model>(CREATE_MEME_QUERY)
            .bind(id)
            .bind(form.name)
            .bind(form.description)
            .bind(s3_path)
            .fetch_one(&self.conn)
            .await?;

        let obj_name = gen_s3_obj_name(model.id);
        let args = &UploadObjectArgs::new(
            BUCKET_NAME,
            obj_name.as_str(),
            file.file.path().to_str().unwrap(),
        )?;

        self.s3.upload_object(args).await?;

        sqlx::query(MARK_AS_DONE_QUERY)
            .bind(model.id)
            .execute(&self.conn)
            .await?;

        Ok(model)
    }

    pub async fn update(&self, id: Uuid, form: UpdateForm) -> Result<Model, errors::AppError> {
        match sqlx::query_as::<_, Model>(UPDATE_MEME_QUERY)
            .bind(id)
            .bind(form.name)
            .bind(form.description)
            .fetch_one(&self.conn)
            .await
        {
            Ok(model) => Ok(model),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), errors::AppError> {
        match sqlx::query(DELETE_MEME_QUERY)
            .bind(id)
            .execute(&self.conn)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

fn gen_s3_path(meme_id: Uuid) -> String {
    format!("memes/{}", meme_id.to_string())
}

fn gen_s3_obj_name(meme_id: Uuid) -> String {
    format!("{}", meme_id.to_string())
}
