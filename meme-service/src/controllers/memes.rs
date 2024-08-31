use crate::common::errors::AppError;
use crate::modules::memes;
use crate::AppData;
use actix_web::{
    body::BodySize,
    web::{self, Bytes},
    HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize)]
struct ListQueryParams {
    limit: Option<u16>,
    offset: Option<u16>,
}

pub async fn list(
    _req: HttpRequest,
    shared: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
    let memes = shared.mods.memes.list().await?;
    Ok(web::Json(memes))
}

pub async fn get(
    _req: HttpRequest,
    shared: web::Data<AppData>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let id = Uuid::parse_str(&path.into_inner())?;
    let meme = shared.mods.memes.get(id).await?;

    Ok(web::Json(meme))
}

pub async fn create(
    _req: HttpRequest,
    shared: web::Data<AppData>,
    form: web::Json<memes::model::CreateForm>,
) -> Result<impl Responder, AppError> {
    form.validate()?;
    let meme = shared.mods.memes.create(form.into_inner()).await?;

    Ok(web::Json(meme))
}

pub async fn update(
    _req: HttpRequest,
    shared: web::Data<AppData>,
    path: web::Path<String>,
    form: web::Json<memes::model::UpdateForm>,
) -> Result<impl Responder, AppError> {
    let id = Uuid::parse_str(&path.into_inner())?;
    let meme = shared.mods.memes.update(id, form.into_inner()).await?;

    Ok(web::Json(meme))
}

pub async fn delete(
    _req: HttpRequest,
    shared: web::Data<AppData>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let id = Uuid::parse_str(&path.into_inner())?;
    shared.mods.memes.delete(id).await?;

    Ok(HttpResponse::Ok())
}
