mod common;
mod controllers;
mod modules;
mod utils;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use common::errors::AppError;
use minio::s3;
use minio::s3::args::{BucketExistsArgs, MakeBucketArgs};
use minio::s3::client::{Client, ClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use modules::memes;
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error};
use utils::s3signer::S3Signer;

const BUCKET_NAME: &str = "memes";

pub struct S3 {
    base_url: String,
    access_key: String,
    secret_key: String,
}

pub struct Config {
    pub postgres_dsn: String,
    pub s3: S3,
}

impl Config {
    pub fn build_from_env() -> Result<Config, &'static str> {
        let postgres_dsn = match env::var("POSTGRES_DSN") {
            Ok(dsn) => dsn,
            Err(_) => return Err("environment variable POSTGRES_DSN is not defined"),
        };
        let base_url = match env::var("S3_BASE_URL") {
            Ok(url) => url,
            Err(_) => return Err("environment variable S3_BASE_URL is not defined"),
        };
        let access_key = match env::var("S3_ACCESS_KEY") {
            Ok(key) => key,
            Err(_) => return Err("environment variable S3_ACCESS_KEY is not defined"),
        };
        let secret_key = match env::var("S3_SECRET_KEY") {
            Ok(key) => key,
            Err(_) => return Err("environment variable S3_SECRET_KEY is not defined"),
        };

        Ok(Config {
            postgres_dsn,
            s3: S3 {
                base_url,
                access_key,
                secret_key,
            },
        })
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

    let s3_client = connect_to_s3(&cfg.s3)?;
    setup_bucket(&s3_client).await?;
    let signer = s3_signer(&cfg.s3);
    let meme_module = modules::memes::Module::new(pool, s3_client, signer);

    let app_data = AppData {
        mods: Modules { memes: meme_module },
    };

    // custom `Json` extractor configuration
    let json_cfg = web::JsonConfig::default()
        // limit request payload size
        .limit(4096)
        // use custom error handler
        .error_handler(|err, _req| AppError::json_validation_error(err.to_string()).into());

    HttpServer::new(move || {
        App::new()
            .app_data(json_cfg.clone())
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

fn connect_to_s3(conf: &S3) -> Result<Client, s3::error::Error> {
    let base_url = conf.base_url.parse::<BaseUrl>()?;

    let static_provider =
        StaticProvider::new(conf.access_key.as_str(), conf.secret_key.as_str(), None);

    let client = ClientBuilder::new(base_url.clone())
        .provider(Some(Box::new(static_provider)))
        .build()?;

    Ok(client)
}

async fn setup_bucket(client: &Client) -> Result<(), s3::error::Error> {
    // Check 'bucket_name' bucket exist or not.
    let exists: bool = client
        .bucket_exists(&BucketExistsArgs::new(&BUCKET_NAME).unwrap()) // unwrap cause im not dumb
        .await?;

    // Make 'bucket_name' bucket if not exist.
    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(&BUCKET_NAME).unwrap())
            .await?;
    }

    Ok(())
}

fn s3_signer(conf: &S3) -> S3Signer {
    S3Signer::new(conf.base_url.clone(), &conf.access_key, &conf.secret_key)
}
