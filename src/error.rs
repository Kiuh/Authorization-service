use thiserror::Error;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde_json::json;
use std::fmt;

/// A trait that allows converting an implemented type to an HTTP error
pub trait ResponseError: fmt::Display {
    /// Error code specified in the HTTP response
    fn code(&self) -> u32;

    /// The message specified in the HTTP response
    fn message(&self) -> String {
        self.to_string()
    }

    fn http_status_400(&self) -> ResponseErrorData {
        ResponseErrorData {
            http_status_code: StatusCode::BAD_REQUEST,
            code: self.code(),
            message: self.message(),
        }
    }

    fn http_status_404(&self) -> ResponseErrorData {
        ResponseErrorData {
            http_status_code: StatusCode::NOT_FOUND,
            code: self.code(),
            message: self.message(),
        }
    }

    fn http_status_500(&self) -> ResponseErrorData {
        ResponseErrorData {
            http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            code: self.code(),
            message: self.message(),
        }
    }
}

#[derive(Debug)]
pub struct ResponseErrorData {
    http_status_code: StatusCode,
    code: u32,
    message: String,
}

impl actix_web::error::ResponseError for ResponseErrorData {
    fn status_code(&self) -> StatusCode {
        self.http_status_code
    }

    fn error_response(&self) -> HttpResponse {
        let body = json!({
            "code": self.code,
            "message": self.message,
        })
        .to_string();

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
    }
}

// Required for actix_web::error::ResponseError
impl fmt::Display for ResponseErrorData {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

pub type Result<T = ()> = std::result::Result<T, ServerError>;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed connect to database: {0}")]
    DatabaseConnection(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Failed to log in")]
    LogInFailed,

    #[error("Failed to register")]
    RegisterFailed,
}

impl ResponseError for ServerError {
    fn code(&self) -> u32 {
        match self {
            ServerError::DatabaseConnection(_) => 1,
            ServerError::Database(_) => 2,
            ServerError::LogInFailed => 3,
            ServerError::RegisterFailed => 4,
        }
    }
}
