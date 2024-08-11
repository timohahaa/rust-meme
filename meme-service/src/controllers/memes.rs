use actix_web::{web, HttpRequest, Responder};
use uuid::Uuid;

pub async fn list(_req: HttpRequest) -> impl Responder {
    "list memes"
}

pub async fn get(_req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let id = Uuid::parse_str(&path.into_inner());
    match id {
        Ok(uid) => format!("get meme with id {}", uid),
        Err(e) => format!("an error occured: {}", e),
    }
}

pub async fn create(_req: HttpRequest) -> impl Responder {
    "create meme"
}

pub async fn update(_req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let id = Uuid::parse_str(&path.into_inner());
    match id {
        Ok(uid) => format!("update meme with id {}", uid),
        Err(e) => format!("an error occured: {}", e),
    }
}

pub async fn delete(_req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let id = Uuid::parse_str(&path.into_inner());
    match id {
        Ok(uid) => format!("delete meme with id {}", uid),
        Err(e) => format!("an error occured: {}", e),
    }
}
