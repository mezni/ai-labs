use thiserror::Error;

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("Invalid persisted domain data")]
    InvalidData,

    #[error("Invalid persisted identifier")]
    InvalidIdentifier,

    #[error("Invalid persisted monetary value")]
    InvalidMoney,
}