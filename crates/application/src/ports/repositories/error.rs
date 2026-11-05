use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Storage is temporarily unavailable")]
    Unavailable,

    #[error("A conflicting record already exists")]
    Conflict,

    #[error("Repository operation failed")]
    Unexpected,
}