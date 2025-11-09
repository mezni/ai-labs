use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error")]
    Internal,
}

impl AppError {
    pub fn to_response(&self) -> ErrorResponse {
        match self {
            AppError::Database(msg) => ErrorResponse {
                code: "DATABASE_ERROR".to_string(),
                message: msg.clone(),
                details: None,
            },
            AppError::Validation(msg) => ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: msg.clone(),
                details: None,
            },
            AppError::Unauthorized => ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            },
            AppError::NotFound(resource) => ErrorResponse {
                code: "NOT_FOUND".to_string(),
                message: format!("{} not found", resource),
                details: None,
            },
            AppError::Internal => ErrorResponse {
                code: "INTERNAL_ERROR".to_string(),
                message: "Internal server error".to_string(),
                details: None,
            },
        }
    }
}
