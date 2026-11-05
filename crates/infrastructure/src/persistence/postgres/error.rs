use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("PostgreSQL connection failed")]
    Connection(#[source] sqlx::Error),

    #[error("PostgreSQL health check failed")]
    HealthCheckFailed,

    #[error("Database migration failed")]
    Migration(#[source] sqlx::migrate::MigrateError),
}