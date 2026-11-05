use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),
    #[error("Query failed: {0}")]
    QueryError(String),
    #[error("Migration failed: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("Transaction failed: {0}")]
    TransactionError(#[from] sqlx::Error),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;