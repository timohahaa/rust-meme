use std::{env, error::Error};

use meme_service::Config;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config {
        postgres_dsn: env::var("POSTGRES_DSN")?,
    };

    meme_service::run(cfg).await
}
