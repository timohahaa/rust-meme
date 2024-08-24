use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
pub struct Module {
    conn: Pool<Postgres>,
}

impl Module {
    pub async fn new(conn: Pool<Postgres>) -> Module {
        Module { conn }
    }
}
