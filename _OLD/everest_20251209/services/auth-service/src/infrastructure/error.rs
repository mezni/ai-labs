use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Keycloak error: {0}")]
    KeycloakError(String),

    #[error("User already exists: {0}")]
    UserAlreadyExists(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            DomainError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            DomainError::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            DomainError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            DomainError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            DomainError::AuthenticationError(_) => {
                (StatusCode::UNAUTHORIZED, "AUTHENTICATION_ERROR")
            }
            DomainError::KeycloakError(_) => (StatusCode::BAD_GATEWAY, "KEYCLOAK_ERROR"),
            DomainError::UserAlreadyExists(_) => (StatusCode::CONFLICT, "USER_ALREADY_EXISTS"),
            DomainError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            DomainError::NotFound(_) => StatusCode::NOT_FOUND,
            DomainError::ValidationError(_) => StatusCode::BAD_REQUEST,
            DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DomainError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DomainError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            DomainError::KeycloakError(_) => StatusCode::BAD_GATEWAY,
            DomainError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
