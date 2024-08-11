use meme_service::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = Config {};

    meme_service::run(cfg).await
}
