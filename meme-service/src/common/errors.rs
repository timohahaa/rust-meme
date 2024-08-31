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
    JsonValidationError,
    ValidationError,
    InvalidUUIDError,
}

impl AppErrorType {
    fn code(&self) -> u32 {
        match self {
            AppErrorType::DbError => 500001,
            AppErrorType::JsonValidationError => 400001,
            AppErrorType::ValidationError => 400002,
            AppErrorType::InvalidUUIDError => 400003,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: Option<String>,
    pub code: u32,
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

    pub fn json_validation_error(err: String) -> Self {
        let et = AppErrorType::JsonValidationError;
        AppError {
            message: Some(err),
            code: et.code(),
            error_type: et,
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
            AppErrorType::ValidationError => StatusCode::BAD_REQUEST,
            AppErrorType::JsonValidationError => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> AppError {
        let et = AppErrorType::DbError;
        AppError {
            message: Some(err.to_string()),
            code: et.code(),
            error_type: et,
        }
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> AppError {
        let et = AppErrorType::InvalidUUIDError;
        AppError {
            message: Some(format!("invalid UUID format: {}", err.to_string())),
            code: et.code(),
            error_type: et,
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errs: validator::ValidationErrors) -> AppError {
        let et = AppErrorType::ValidationError;
        AppError {
            message: Some(format!("validation error: {}", errs.to_string())),
            code: et.code(),
            error_type: et,
        }
    }
}
