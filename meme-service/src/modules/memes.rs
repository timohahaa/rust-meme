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

    pub async fn list(&self) -> &'static str {
        "list memes"
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
