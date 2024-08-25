use meme_service::Config;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::build_from_env()?;

    meme_service::run(cfg).await
}
