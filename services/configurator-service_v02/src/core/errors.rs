use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {source}")]
    ConfigError {
        #[from]
        source: config::ConfigError,
    },

    #[error("I/O error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Validation error: {details}")]
    ValidationError { details: String },

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ConfigError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IoError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        // JSON error response
        let body = serde_json::json!({
            "error": self.to_string(),
            "code": status.as_u16(),
        });

        HttpResponse::build(status).json(body)
    }
}

// Convenient constructor functions
impl AppError {
    pub fn validation<D: Into<String>>(details: D) -> Self {
        Self::ValidationError {
            details: details.into(),
        }
    }

    pub fn not_found<R: Into<String>>(resource: R) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }
}
