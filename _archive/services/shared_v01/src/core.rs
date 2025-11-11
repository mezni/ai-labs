use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Optional details
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(msg: &str) -> Self {
        Self {
            code: "ERROR".into(),
            message: msg.to_string(),
            details: None,
        }
    }
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
            AppError::Database(msg) => Self::err("DATABASE_ERROR", msg),
            AppError::Validation(msg) => Self::err("VALIDATION_ERROR", msg),
            AppError::Unauthorized => Self::err("UNAUTHORIZED", "Authentication required"),
            AppError::NotFound(res) => Self::err("NOT_FOUND", &format!("{} not found", res)),
            AppError::Internal => Self::err("INTERNAL_ERROR", "Internal server error"),
        }
    }

    fn err(code: &str, msg: &str) -> ErrorResponse {
        ErrorResponse {
            code: code.into(),
            message: msg.into(),
            details: None,
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: &ErrorResponse) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.clone()),
        }
    }
}
