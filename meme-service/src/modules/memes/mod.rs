mod model;
mod queries;

use futures::TryStreamExt;
use model::Model;
use queries::list_memes_query;
use sqlx::{Pool, Postgres};
use std::error::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct Module {
    conn: Pool<Postgres>,
}

impl Module {
    pub async fn new(conn: Pool<Postgres>) -> Module {
        Module { conn }
    }

    pub async fn list(&self) -> Result<Vec<Model>, Box<dyn Error>> {
        let mut rows = sqlx::query_as(list_memes_query).fetch(&self.conn);
        let mut memes = vec![];

        while let Some(meme) = rows.try_next().await? {
            memes.push(meme)
        }

        Ok(memes)
    }

    pub async fn get(&self, id: Uuid) -> String {
        format!("get meme with id {}", id)
    }

    pub async fn create(&self) -> String {
        format!("create meme with id {}", Uuid::new_v4())
    }

    pub async fn update(&self, id: Uuid) -> String {
        format!("update meme with id {}", id)
    }

    pub async fn delete(&self, id: Uuid) -> String {
        format!("delete meme with id {}", id)
    }
}
