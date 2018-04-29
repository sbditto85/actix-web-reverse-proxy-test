// Stolen and modified from https://github.com/JustinRyanH/actix-oauth2/blob/experimental_redis/src/errors.rs
use std::error::Error;
use std::fmt;

use http;

use actix_web::client::SendRequestError;
use actix_web::http::StatusCode;
use actix_web::{self, HttpResponse};

// pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ActixWeb(actix_web::Error),
    ActixSendError(actix_web::client::SendRequestError),
    HttpError(http::Error),
    Lazy(String),
}

// impl AppError {
//     pub fn lazy<S: Into<String>>(s: S) -> AppError {
//         AppError::Lazy(s.into())
//     }
// }

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AppError::ActixWeb(ref err) => write!(f, "ActixWeb Error: {:?}", err),
            &AppError::ActixSendError(ref err) => write!(f, "ActixSend Error: {:?}", err),
            &AppError::HttpError(ref err) => write!(f, "Http Error: {}", err),
            &AppError::Lazy(ref err) => write!(f, "Lazy Error: {}", err),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::ActixWeb(_) => "Errors invloving the Async Web Rust Framework",
            AppError::ActixSendError(_) => "Couldn't connect to host",
            AppError::HttpError(_) => "Error with http stuff",
            AppError::Lazy(_) => "Errors I am too lazy to specify",
        }
    }
}

impl From<http::Error> for AppError {
    fn from(v: http::Error) -> AppError {
        AppError::HttpError(v)
    }
}

impl From<String> for AppError {
    fn from(v: String) -> AppError {
        AppError::Lazy(v)
    }
}

impl From<actix_web::Error> for AppError {
    fn from(v: actix_web::Error) -> AppError {
        AppError::ActixWeb(v)
    }
}

impl From<SendRequestError> for AppError {
    fn from(v: SendRequestError) -> AppError {
        AppError::ActixSendError(v)
    }
}

impl From<actix_web::error::PayloadError> for AppError {
    fn from(v: actix_web::error::PayloadError) -> AppError {
        AppError::Lazy(format!("PayloadError {:?}", v))
    }
}
