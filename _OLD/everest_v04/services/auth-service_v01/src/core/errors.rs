use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Keycloak error: {0}")]
    KeycloakError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Max resend attempts reached")]
    MaxResendAttemptsReached,

    #[error("Resend cooldown active")]
    ResendCooldownActive,
}

impl AppError {
    pub fn to_response(&self) -> ErrorResponse {
        ErrorResponse {
            error: self.error_type(),
            message: self.to_string(),
            details: None,
        }
    }

    pub fn error_type(&self) -> String {
        match self {
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::KeycloakError(_) => "KEYCLOAK_ERROR",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::AlreadyExists(_) => "ALREADY_EXISTS",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::InternalError(_) => "INTERNAL_ERROR",
            AppError::TokenExpired => "TOKEN_EXPIRED",
            AppError::InvalidToken => "INVALID_TOKEN",
            AppError::MaxResendAttemptsReached => "MAX_RESEND_ATTEMPTS",
            AppError::ResendCooldownActive => "RESEND_COOLDOWN_ACTIVE",
        }
        .to_string()
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) | AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
            AppError::Unauthorized(_) | AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::TokenExpired => StatusCode::GONE,
            AppError::MaxResendAttemptsReached | AppError::ResendCooldownActive => {
                StatusCode::TOO_MANY_REQUESTS
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_response())
    }

    fn status_code(&self) -> StatusCode {
        self.status_code()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::KeycloakError(err.to_string())
    }
}