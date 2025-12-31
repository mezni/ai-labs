use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // Database errors
    DatabaseError(String),
    NotFound(String),
    Conflict(String),

    // Keycloak errors
    KeycloakError(String),
    NetworkError(String),

    // Authentication errors
    Unauthorized(String),
    Forbidden(String),
    InvalidCredentials,
    TokenExpired,
    InvalidToken,

    // Validation errors
    ValidationError(String),
    BadRequest(String),

    // Registration errors
    VerificationExpired,
    VerificationFailed(String),
    AlreadyVerified,
    TooManyResendAttempts,

    // Invitation errors
    InvitationExpired,
    InvitationAlreadyAccepted,
    InvalidInvitationCode,

    // General errors
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Self::KeycloakError(msg) => write!(f, "Keycloak error: {}", msg),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            Self::InvalidCredentials => write!(f, "Invalid credentials"),
            Self::TokenExpired => write!(f, "Token expired"),
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Self::VerificationExpired => write!(f, "Verification link expired"),
            Self::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            Self::AlreadyVerified => write!(f, "Already verified"),
            Self::TooManyResendAttempts => write!(f, "Too many resend attempts"),
            Self::InvitationExpired => write!(f, "Invitation expired"),
            Self::InvitationAlreadyAccepted => write!(f, "Invitation already accepted"),
            Self::InvalidInvitationCode => write!(f, "Invalid invitation code"),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DatabaseError(_) | Self::InternalError(_) | Self::NetworkError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::KeycloakError(_) => StatusCode::BAD_GATEWAY,
            Self::Unauthorized(_) | Self::InvalidCredentials | Self::TokenExpired | Self::InvalidToken => {
                StatusCode::UNAUTHORIZED
            }
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::ValidationError(_) | Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::VerificationExpired | Self::InvitationExpired => StatusCode::GONE,
            Self::VerificationFailed(_) => StatusCode::BAD_REQUEST,
            Self::AlreadyVerified | Self::InvitationAlreadyAccepted => StatusCode::CONFLICT,
            Self::TooManyResendAttempts => StatusCode::TOO_MANY_REQUESTS,
            Self::InvalidInvitationCode => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_type = match self {
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Conflict(_) => "CONFLICT",
            Self::KeycloakError(_) => "KEYCLOAK_ERROR",
            Self::NetworkError(_) => "NETWORK_ERROR",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::InvalidToken => "INVALID_TOKEN",
            Self::ValidationError(_) => "VALIDATION_ERROR",
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::VerificationExpired => "VERIFICATION_EXPIRED",
            Self::VerificationFailed(_) => "VERIFICATION_FAILED",
            Self::AlreadyVerified => "ALREADY_VERIFIED",
            Self::TooManyResendAttempts => "TOO_MANY_RESEND_ATTEMPTS",
            Self::InvitationExpired => "INVITATION_EXPIRED",
            Self::InvitationAlreadyAccepted => "INVITATION_ALREADY_ACCEPTED",
            Self::InvalidInvitationCode => "INVALID_INVITATION_CODE",
            Self::InternalError(_) => "INTERNAL_ERROR",
        };

        let response = ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
            details: None,
        };

        HttpResponse::build(status).json(response)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound("Resource not found".into()),
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    Self::Conflict(format!("Constraint violation: {}", constraint))
                } else {
                    Self::DatabaseError(db_err.to_string())
                }
            }
            _ => Self::DatabaseError(err.to_string()),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;