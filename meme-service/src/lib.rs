use actix_web::{web, App, HttpResponse, HttpServer};

mod controllers;

pub struct Config {}

pub async fn run(_cfg: Config) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(|| async { HttpResponse::Ok() }))
            .service(
                web::scope("/api")
                    .service(web::scope("/memes").configure(controllers::memes_service)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
