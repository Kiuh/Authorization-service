use thiserror::Error;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde_json::json;
use std::fmt;

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

    #[error("Couldn't find user with login/email {0}")]
    UserNotFound(String),

    #[error("Failed to initialize mail client: {0}")]
    MailInit(String),

    #[error("Failed to send e-mail: {0}")]
    MailSend(String),

    #[error("Wrong nonce")]
    WrongNonce,

    #[error("Failed to decode {parameter_name} from base58")]
    Base58Decode { parameter_name: String },

    #[error("Failed to decode {parameter_name} from rsa")]
    RsaDecode { parameter_name: String },

    #[error("Wrong encoding for {parameter_name}: expected UTF-8")]
    WrongTextEncoding { parameter_name: String },

    #[error("Wrong access code")]
    WrongAccessCode,

    #[error("Wrong request")]
    WrongRequest,

    #[error("Unsupported HTTP method")]
    UnsupportedHttpMethod,

    #[error("Failed to send request to core service")]
    SendRequestToCoreService,

    #[error("Wrong signature")]
    WrongSignature,
}

pub trait ResponseError: fmt::Display {
    fn code(&self) -> u32;

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

pub trait IntoHttpResult<T> {
    fn into_http_400(self) -> actix_web::Result<T>;
    fn into_http_404(self) -> actix_web::Result<T>;
    fn into_http_500(self) -> actix_web::Result<T>;
}

impl<T> IntoHttpResult<T> for Result<T> {
    fn into_http_400(self) -> actix_web::Result<T> {
        self.map_err(|e| e.http_status_400().into())
    }

    fn into_http_404(self) -> actix_web::Result<T> {
        self.map_err(|e| e.http_status_404().into())
    }

    fn into_http_500(self) -> actix_web::Result<T> {
        self.map_err(|e| e.http_status_500().into())
    }
}

impl<T> IntoHttpResult<T> for ServerError {
    fn into_http_400(self) -> actix_web::Result<T> {
        Err(self).into_http_400()
    }

    fn into_http_404(self) -> actix_web::Result<T> {
        Err(self).into_http_404()
    }

    fn into_http_500(self) -> actix_web::Result<T> {
        Err(self).into_http_500()
    }
}

impl ResponseError for ServerError {
    fn code(&self) -> u32 {
        match self {
            ServerError::DatabaseConnection(_) => 1,
            ServerError::Database(_) => 2,
            ServerError::LogInFailed => 3,
            ServerError::RegisterFailed => 4,
            ServerError::UserNotFound(_) => 5,
            ServerError::MailInit(_) => 9,
            ServerError::MailSend(_) => 10,
            ServerError::WrongNonce => 11,
            ServerError::Base58Decode { .. } => 12,
            ServerError::RsaDecode { .. } => 13,
            ServerError::WrongTextEncoding { .. } => 14,
            ServerError::WrongAccessCode { .. } => 15,
            ServerError::WrongRequest => 16,
            ServerError::UnsupportedHttpMethod => 17,
            ServerError::SendRequestToCoreService => 18,
            ServerError::WrongSignature => 19,
        }
    }
}
