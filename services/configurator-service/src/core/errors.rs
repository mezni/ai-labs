use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

/// Custom application error type
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ConfigError(_)
            | AppError::IoError(_)
            | AppError::DatabaseError(_)
            | AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_error_response())
    }
}

impl AppError {
    /// Creates a new validation error
    pub fn validation<D: Into<String>>(details: D) -> Self {
        Self::ValidationError(details.into())
    }

    /// Creates a new not found error
    pub fn not_found<R: Into<String>>(resource: R) -> Self {
        Self::NotFound(resource.into())
    }

    /// Creates a new database error
    pub fn database<D: Into<String>>(details: D) -> Self {
        Self::DatabaseError(details.into())
    }

    // Helper function to generate error response
    fn as_error_response(&self) -> serde_json::Value {
        serde_json::json!({
            "error": self.to_string(),
            "code": self.status_code().as_u16(),
        })
    }
}
