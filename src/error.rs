use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// Central app error type, mapped to HTTP responses by IntoResponse.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Database Error: {0}")]
    Database(Box<surrealdb::Error>),

    #[error("Internal Server Error: {0}")]
    Internal(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Invalid credentials")]
    InvalidCredentials,
}

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        Self::Database(Box::new(err))
    }
}

// Convert any error into a JSON response with the right status code.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Database(err) => {
                eprintln!("Database error: {err}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }

            Self::Internal(message) => {
                eprintln!("Internal server error: {message}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }

            Self::NotFound(message) => {
                (StatusCode::NOT_FOUND, message)
            }

            Self::InvalidInput(message) => {
                (StatusCode::BAD_REQUEST, message)
            }

            Self::InvalidCredentials => {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid email or password".to_string(),
                )
            }
        };

        (
            status,
            Json(json!({
                "status": status.as_u16(),
                "message": message,
            })),
        )
            .into_response()
    }
}
