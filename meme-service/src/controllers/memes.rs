use crate::AppData;
use crate::{common::render, modules::memes};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, Responder, Result};
use uuid::Uuid;

pub async fn list(_req: HttpRequest, shared: web::Data<AppData>) -> Result<impl Responder> {
    match shared.mods.memes.list().await {
        Ok(list) => return Ok(web::Json(list)),
        Err(e) => {
            return Err(render::error(
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };
}

pub async fn get(
    _req: HttpRequest,
    shared: web::Data<AppData>,
    path: web::Path<String>,
) -> impl Responder {
    let id = Uuid::parse_str(&path.into_inner());
    //match id {
    //    Ok(uid) => return shared.mods.memes.get(uid).await,
    //    Err(e) => format!("an error occured: {}", e),
    //}

    ""
}

pub async fn create(
    _req: HttpRequest,
    shared: web::Data<AppData>,
    form: web::Json<memes::model::CreateForm>,
) -> impl Responder {
    shared.mods.memes.create(form.into_inner()).await;

    ""
}

pub async fn update(
    _req: HttpRequest,
    shared: web::Data<AppData>,
    path: web::Path<String>,
) -> impl Responder {
    let id = Uuid::parse_str(&path.into_inner());
    //match id {
    //    Ok(uid) => shared.mods.memes.update(uid).await,
    //    Err(e) => format!("an error occured: {}", e),
    //};
    ""
}

pub async fn delete(
    _req: HttpRequest,
    shared: web::Data<AppData>,
    path: web::Path<String>,
) -> impl Responder {
    let id = Uuid::parse_str(&path.into_inner());
    //match id {
    //    Ok(uid) => shared.mods.memes.delete(uid).await,
    //    Err(e) => format!("an error occured: {}", e),
    //}
    ""
}
