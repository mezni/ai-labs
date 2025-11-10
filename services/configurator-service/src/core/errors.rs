use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
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

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ConfigError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::IoError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ServiceError::NotFound { .. } => StatusCode::NOT_FOUND,
            ServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        // Create JSON error response
        let body = serde_json::json!({
            "error": self.to_string(),
            "code": status.as_u16(),
        });

        HttpResponse::build(status).json(body)
    }
}

// Convenient constructor functions
impl ServiceError {
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
