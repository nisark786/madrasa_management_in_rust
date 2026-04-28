use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("resource not found")]
    NotFound,
    #[error("validation failed")]
    Validation(String),
    #[error("conflict")]
    Conflict(&'static str),
    #[error("database error")]
    Db,
    #[error("internal server error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "AUTH_001", "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "AUTH_002", "Forbidden".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "GEN_404", "Not found".to_string()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "VAL_001", msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "GEN_409", msg.to_string()),
            AppError::Db => (StatusCode::INTERNAL_SERVER_ERROR, "DB_500", "Database error".to_string()),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "GEN_500",
                "Internal server error".to_string(),
            ),
        };

        let body = Json(ErrorBody { code, message });
        (status, body).into_response()
    }
}
