// エラー定義
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Internal server error")]
    InternalError(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InternalServerError(_) => {
                tracing::error!("Internal server error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::InternalError(_) => {
                tracing::error!("Internal error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::DatabaseError(_) => {
                tracing::error!("Database error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, "Forbidden"),
        };

        (status, message).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
