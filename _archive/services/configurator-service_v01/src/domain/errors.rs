use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid network type: {0}")]
    InvalidNetworkType(String),

    #[error("Network name cannot be empty")]
    EmptyName,

    #[error("Network name too long: {0} characters (max: {1})")]
    NameTooLong(usize, usize),

    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("Phone number too long: {0} characters (max: {1})")]
    PhoneTooLong(usize, usize),

    #[error("Network not found: {0}")]
    NetworkNotFound(i32),

    #[error("Network name already exists: {0}")]
    DuplicateName(String),

    #[error("Created by user cannot be empty")]
    EmptyCreatedBy,

    #[error("Database error: {0}")]
    Database(String),
}

impl DomainError {
    pub fn to_api_error(&self) -> shared::core::ErrorResponse {
        shared::core::ErrorResponse {
            code: self.error_code().to_string(),
            message: self.to_string(),
            details: None,
        }
    }

    fn error_code(&self) -> &str {
        match self {
            DomainError::InvalidNetworkType(_) => "INVALID_NETWORK_TYPE",
            DomainError::EmptyName => "EMPTY_NAME",
            DomainError::NameTooLong(_, _) => "NAME_TOO_LONG",
            DomainError::InvalidEmail(_) => "INVALID_EMAIL",
            DomainError::PhoneTooLong(_, _) => "PHONE_TOO_LONG",
            DomainError::NetworkNotFound(_) => "NETWORK_NOT_FOUND",
            DomainError::DuplicateName(_) => "DUPLICATE_NAME",
            DomainError::EmptyCreatedBy => "EMPTY_CREATED_BY",
            DomainError::Database(_) => "DATABASE_ERROR",
        }
    }
}
