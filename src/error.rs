use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] libsql::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_msg, details) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone(), None),
            AppError::Validation(errs) => {
                let details = serde_json::to_value(errs).ok();
                (StatusCode::BAD_REQUEST, "Validation failed".into(), details)
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone(), None),
            AppError::Database(_) | AppError::Internal(_) => {
                tracing::error!("Internal error: {}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                    None,
                )
            }
        };

        (
            status,
            Json(ErrorResponse {
                error: error_msg,
                details,
            }),
        )
            .into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
