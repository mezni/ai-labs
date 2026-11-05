use bio_com_application::ports::repositories::error::RepositoryError;

pub fn map_database_error(
    error: sqlx::Error,
) -> RepositoryError {
    match error {
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("23505") =>
        {
            RepositoryError::Conflict
        }

        sqlx::Error::PoolTimedOut
        | sqlx::Error::PoolClosed
        | sqlx::Error::Io(_)
        | sqlx::Error::Tls(_) => {
            RepositoryError::Unavailable
        }

        _ => RepositoryError::Unexpected,
    }
}