use crate::infrastructure::keycloak_client;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Keycloak(String),
    Network(String),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    Internal(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::Keycloak(e) => write!(f, "Keycloak error: {}", e),
            Self::Network(e) => write!(f, "Network error: {}", e),
            Self::NotFound(e) => write!(f, "Not found: {}", e),
            Self::BadRequest(e) => write!(f, "Bad request: {}", e),
            Self::Unauthorized(e) => write!(f, "Unauthorized: {}", e),
            Self::Forbidden(e) => write!(f, "Forbidden: {}", e),
            Self::Conflict(e) => write!(f, "Conflict: {}", e),
            Self::Internal(e) => write!(f, "Internal error: {}", e),
            Self::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            Self::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            Self::Keycloak(_) => (StatusCode::BAD_GATEWAY, "Authentication service error"),
            Self::Network(_) => (StatusCode::BAD_GATEWAY, "Network error"),
            Self::NotFound(m) => (StatusCode::NOT_FOUND, m.as_str()),
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m.as_str()),
            Self::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m.as_str()),
            Self::Forbidden(m) => (StatusCode::FORBIDDEN, m.as_str()),
            Self::Conflict(m) => (StatusCode::CONFLICT, m.as_str()),
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
            Self::ValidationError(m) => (StatusCode::BAD_REQUEST, m.as_str()),
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": message,
            "details": self.to_string()
        }))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl From<keycloak_client::AppError> for AppError {
    fn from(err: keycloak_client::AppError) -> Self {
        AppError::Keycloak(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
