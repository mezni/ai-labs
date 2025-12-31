use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // Database errors
    DatabaseError(String),
    
    // Resource errors
    NotFound(String),
    Conflict(String),
    
    // Request errors
    BadRequest(String),
    ValidationError(String),
    
    // Authentication/Authorization errors
    Unauthorized(String),
    Forbidden(String),
    
    // Token errors
    InvalidToken(String),
    ExpiredToken(String),
    
    // Rate limiting
    RateLimitExceeded(String),
    
    // External service errors
    KeycloakError(String),
    EmailServiceError(String),
    SmsServiceError(String),
    
    // Internal errors
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AppError::ExpiredToken(msg) => write!(f, "Expired token: {}", msg),
            AppError::RateLimitExceeded(msg) => write!(f, "Rate limit exceeded: {}", msg),
            AppError::KeycloakError(msg) => write!(f, "Keycloak error: {}", msg),
            AppError::EmailServiceError(msg) => write!(f, "Email service error: {}", msg),
            AppError::SmsServiceError(msg) => write!(f, "SMS service error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_type = self.error_type();
        let error_message = self.to_string();
        
        let response = ErrorResponse {
            error: error_type,
            message: error_message,
            status: status_code.as_u16(),
            details: None,
        };
        
        HttpResponse::build(status_code).json(response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            AppError::ExpiredToken(_) => StatusCode::UNAUTHORIZED,
            AppError::RateLimitExceeded(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::KeycloakError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EmailServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SmsServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl AppError {
    fn error_type(&self) -> String {
        match self {
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::InvalidToken(_) => "INVALID_TOKEN",
            AppError::ExpiredToken(_) => "EXPIRED_TOKEN",
            AppError::RateLimitExceeded(_) => "RATE_LIMIT_EXCEEDED",
            AppError::KeycloakError(_) => "KEYCLOAK_ERROR",
            AppError::EmailServiceError(_) => "EMAIL_SERVICE_ERROR",
            AppError::SmsServiceError(_) => "SMS_SERVICE_ERROR",
            AppError::InternalError(_) => "INTERNAL_ERROR",
        }
        .to_string()
    }
}

// Conversion from domain validation errors
impl From<crate::domain::value_objects::ValidationError> for AppError {
    fn from(err: crate::domain::value_objects::ValidationError) -> Self {
        AppError::ValidationError(err.to_string())
    }
}

// Conversion from SQLx errors
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Resource already exists".to_string())
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

// Conversion from serde_json errors
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(format!("JSON parsing error: {}", err))
    }
}

// Conversion from reqwest errors (for HTTP clients)
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::InternalError(format!("HTTP request error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = AppError::NotFound("User not found".to_string());
        assert_eq!(error.to_string(), "Not found: User not found");
    }

    #[test]
    fn test_error_status_code() {
        assert_eq!(
            AppError::NotFound("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::Unauthorized("test".to_string()).status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::RateLimitExceeded("test".to_string()).status_code(),
            StatusCode::TOO_MANY_REQUESTS
        );
    }

    #[test]
    fn test_error_type() {
        let error = AppError::Conflict("Duplicate entry".to_string());
        assert_eq!(error.error_type(), "CONFLICT");
    }
}