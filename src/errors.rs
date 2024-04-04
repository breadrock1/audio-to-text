use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebError {
    #[error("Failed wile extracting text data: {0}")]
    ExtractTextFromAudio(String),
    #[error("Response error: {0}")]
    ResponseError(String),
}

impl WebError {
    pub fn name(&self) -> String {
        match self {
            WebError::ExtractTextFromAudio(_) => "ExtractTextFromAudio",
            _ => "RuntimeError",
        }
        .to_string()
    }
}

impl From<serde_json::Error> for WebError {
    fn from(value: serde_json::Error) -> Self {
        WebError::ResponseError(value.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::ResponseError(_) => StatusCode::BAD_REQUEST,
            WebError::ExtractTextFromAudio(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };

        HttpResponse::build(status_code).json(response)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SuccessfulResponse {
    pub code: u16,
    pub message: String,
}

impl SuccessfulResponse {
    pub fn ok_response(msg: &str) -> HttpResponse {
        let status_code = StatusCode::OK;
        let response = SuccessfulResponse {
            code: status_code.as_u16(),
            message: msg.to_string(),
        };

        HttpResponse::build(status_code).json(response)
    }
}
