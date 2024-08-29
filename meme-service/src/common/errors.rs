use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use serde::Serialize;
use sqlx;
use std::fmt;

#[derive(Debug, Serialize)]
struct HttpErrorContainer {
    error: HttpError,
}

#[derive(Debug, Serialize)]
struct HttpError {
    message: Option<String>,
}

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    InvalidUUIDError,
}

#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub error_type: AppErrorType,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AppError {
    fn to_json(&self) -> HttpErrorContainer {
        HttpErrorContainer {
            error: HttpError {
                message: self.message.clone(),
            },
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self.to_json())
    }

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::InvalidUUIDError => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> AppError {
        AppError {
            message: Some(err.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> AppError {
        AppError {
            message: Some(format!("invalid UUID format: {}", err.to_string())),
            error_type: AppErrorType::InvalidUUIDError,
        }
    }
}
