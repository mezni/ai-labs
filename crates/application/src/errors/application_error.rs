use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("The requested resource was not found")]
    NotFound,

    #[error("The operation is not allowed")]
    OperationNotAllowed,

    #[error("The request contains invalid data: {0}")]
    Validation(String),

    #[error("The requested operation conflicts with existing data")]
    Conflict,

    #[error("An internal application error occurred")]
    Internal,
}
