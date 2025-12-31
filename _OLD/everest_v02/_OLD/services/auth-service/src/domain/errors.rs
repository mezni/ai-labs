use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid username: {0}")]
    InvalidUsername(String),

    #[error("Registration expired")]
    RegistrationExpired,

    #[error("Registration already verified")]
    AlreadyVerified,
}
