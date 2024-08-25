mod model;
mod queries;

use futures::TryStreamExt;
use model::Model;
use queries::list_memes_query;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};
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
        let mut rows = sqlx::query(list_memes_query)
            .map(|row: PgRow| -> Model {
                Model {
                    id: row.get("meme_id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    s3_path: row.get("s3_path"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            })
            .fetch(&self.conn);

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
