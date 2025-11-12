use actix_web::HttpResponse;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Connection pool error: {0}")]
    PoolError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Failed to create resource: {0}")]
    CreateFailed(String),

    #[error("Failed to update resource: {0}")]
    UpdateFailed(String),

    #[error("Failed to delete resource: {0}")]
    DeleteFailed(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(_) => HttpResponse::NotFound().json(self.to_string()),
            AppError::CreateFailed(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            AppError::UpdateFailed(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            AppError::DeleteFailed(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            AppError::Validation(_) => HttpResponse::BadRequest().json(self.to_string()),
            AppError::ServiceUnavailable(_) => {
                HttpResponse::ServiceUnavailable().json(self.to_string())
            }
            AppError::Authentication(_) => HttpResponse::Unauthorized().json(self.to_string()),
            AppError::Authorization(_) => HttpResponse::Forbidden().json(self.to_string()),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(self.to_string()),
            AppError::Database(e) => {
                error!("Database error: {}", e);
                HttpResponse::InternalServerError().json("Database error occurred")
            }
            AppError::PoolError(e) => {
                error!("Connection pool error: {}", e);
                HttpResponse::ServiceUnavailable().json("Service temporarily unavailable")
            }
            AppError::Internal(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}

// Convenience constructors for common error types
impl AppError {
    pub fn not_found(resource: &str) -> Self {
        AppError::NotFound(format!("{} not found", resource))
    }

    pub fn validation(msg: &str) -> Self {
        AppError::Validation(msg.to_string())
    }

    pub fn authentication(msg: &str) -> Self {
        AppError::Authentication(msg.to_string())
    }

    pub fn authorization(msg: &str) -> Self {
        AppError::Authorization(msg.to_string())
    }

    pub fn bad_request(msg: &str) -> Self {
        AppError::BadRequest(msg.to_string())
    }

    pub fn internal(msg: &str) -> Self {
        AppError::Internal(msg.to_string())
    }
}