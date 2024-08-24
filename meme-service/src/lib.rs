use std::{env, error::Error};

use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod controllers;
mod modules;

pub struct Config {
    pub postgres_dsn: String,
}

impl Config {
    pub fn build_from_env() -> Result<Config, &'static str> {
        let postgres_dsn = match env::var("POSTGRES_DSN") {
            Ok(dsn) => dsn,
            Err(_) => return Err("environment variable POSTGRES_DSN is not defined"),
        };

        Ok(Config { postgres_dsn })
    }
}

pub async fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&cfg.postgres_dsn)
        .await?;

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
    .await?;

    Ok(())
}
