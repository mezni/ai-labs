use actix_web::{HttpResponse, ResponseError};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    MigrationError(sqlx::migrate::MigrateError),
    NotFound(String),
    Validation(String),
    Unauthorized,
    Forbidden,
    Internal(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(e) => write!(f, "Database error: {e}"),
            Self::MigrationError(e) => write!(f, "Migration error: {e}"),
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::Validation(msg) => write!(f, "Validation error: {msg}"),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::Forbidden => write!(f, "Forbidden"),
            Self::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Validation(msg) => HttpResponse::BadRequest().body(msg.clone()),
            Self::Unauthorized => HttpResponse::Unauthorized().finish(),
            Self::Forbidden => HttpResponse::Forbidden().finish(),
            Self::NotFound(msg) => HttpResponse::NotFound().body(msg.clone()),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
