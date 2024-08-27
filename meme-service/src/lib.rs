mod common;
mod controllers;
mod modules;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use modules::memes;
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error};

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

#[derive(Clone)]
struct Modules {
    memes: memes::Module,
}

#[derive(Clone)]
struct AppData {
    mods: Modules,
}

pub async fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&cfg.postgres_dsn)
        .await?;

    let meme_module = modules::memes::Module::new(pool).await;

    let app_data = AppData {
        mods: Modules { memes: meme_module },
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_data.clone()))
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
