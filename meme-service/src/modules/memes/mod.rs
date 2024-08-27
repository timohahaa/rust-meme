mod model;
mod queries;

use futures::TryStreamExt;
use model::{CreateForm, Model, UpdateForm};
use queries::{
    create_meme_query, delete_meme_query, get_meme_query, list_memes_query, update_meme_query,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct Module {
    conn: Pool<Postgres>,
}

impl Module {
    pub async fn new(conn: Pool<Postgres>) -> Module {
        Module { conn }
    }

    pub async fn list(&self) -> Result<Vec<Model>, sqlx::Error> {
        let mut rows = sqlx::query_as(list_memes_query).fetch(&self.conn);
        let mut memes = vec![];

        while let Some(meme) = rows.try_next().await? {
            memes.push(meme)
        }

        Ok(memes)
    }

    pub async fn get(&self, id: Uuid) -> Result<Model, sqlx::Error> {
        sqlx::query_as(get_meme_query)
            .bind(id)
            .fetch_one(&self.conn)
            .await
    }

    pub async fn create(&self, form: CreateForm) -> Result<Model, sqlx::Error> {
        sqlx::query_as(create_meme_query)
            .bind(form.name)
            .bind(form.description)
            .bind(form.s3_path)
            .fetch_one(&self.conn)
            .await
    }

    pub async fn update(&self, id: Uuid, form: UpdateForm) -> Result<Model, sqlx::Error> {
        sqlx::query_as(update_meme_query)
            .bind(id)
            .bind(form.name)
            .bind(form.description)
            .fetch_one(&self.conn)
            .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        match sqlx::query(delete_meme_query)
            .bind(id)
            .execute(&self.conn)
            .await
        {
            Ok(_) => Ok(()),
            Err(sqlx_error) => Err(sqlx_error),
        }
    }
}
