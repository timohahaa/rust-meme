use actix_web::{
    http::{header::ContentType, StatusCode},
    ResponseError,
};
use derive_more::derive::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, Serialize, Deserialize)]
#[serde(rename = "error")]
struct HttpError {
    #[serde(rename(serialize = "message"))]
    msg: String,
    #[serde(skip_serializing)]
    code: StatusCode,
}

impl actix_web::error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(&self)
    }

    fn status_code(&self) -> StatusCode {
        self.code
    }
}

pub fn error(msg: String, code: StatusCode) -> impl ResponseError {
    HttpError { msg, code }
}
