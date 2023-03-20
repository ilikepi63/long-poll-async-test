use axum::{
    http::{StatusCode},
    response::{IntoResponse, Response as ErrorResponse},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl IntoResponse for Error {

    fn into_response(self) -> ErrorResponse {
        let (status, error_message) = match self {
            Error::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            Error::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Error::InvalidArgument(message) => (StatusCode::BAD_REQUEST, message),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}