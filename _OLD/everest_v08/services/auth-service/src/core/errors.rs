use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use sqlx;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    MigrationError(String),

    #[error("Internal server error")]
    InternalError,

    #[error("Keycloak error: {0}")]
    KeycloakError(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Maximum resend attempts reached")]
    MaxResendAttemptsReached,

    #[error("Resend cooldown is still active")]
    ResendCooldownActive,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            // 500 Errors
            AppError::Internal(_)
            | AppError::InternalError
            | AppError::DatabaseError(_)
            | AppError::MigrationError(_)
            | AppError::KeycloakError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            // 409 Errors
            AppError::Conflict(_) | AppError::AlreadyExists(_) => StatusCode::CONFLICT,

            // 400 Errors
            AppError::BadRequest(_) | AppError::ValidationError(_) => StatusCode::BAD_REQUEST,

            // 404 Errors
            AppError::NotFound(_) => StatusCode::NOT_FOUND,

            // 401 Errors
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,

            // 410 Errors (Gone)
            AppError::TokenExpired => StatusCode::GONE,

            // 429 Errors (Rate Limiting)
            AppError::MaxResendAttemptsReached | AppError::ResendCooldownActive => {
                StatusCode::TOO_MANY_REQUESTS
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        // Ensure tracing is imported in your crate or use log::error!
        tracing::error!("{}", self.to_string());

        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.to_string(),
        })
    }
}

// Map the Keycloak specific errors to our Global AppError
impl From<crate::infrastructure::keycloak_client::AppError> for AppError {
    fn from(err: crate::infrastructure::keycloak_client::AppError) -> Self {
        use crate::infrastructure::keycloak_client::AppError as K;
        match err {
            K::NotFound(m) => AppError::NotFound(m),
            K::Unauthorized(m) => AppError::Unauthorized(m),
            K::AuthenticationError(m) => AppError::Unauthorized(m),
            K::NetworkError(m) => AppError::Internal(format!("Network failure: {}", m)),
            K::KeycloakError(m) => AppError::KeycloakError(m),
        }
    }
}
