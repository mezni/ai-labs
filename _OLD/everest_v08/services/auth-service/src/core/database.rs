use crate::core::errors::AppError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::{error, info, instrument};

/// Creates a new PostgreSQL connection pool.
/// The #[instrument] macro automatically creates a tracing span,
/// but we skip database_url to avoid logging credentials.
#[instrument(skip(database_url))]
pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    info!("Creating database connection pool");

    PgPoolOptions::new()
        .max_connections(crate::core::constants::MAX_DB_CONNECTIONS)
        .min_connections(crate::core::constants::MIN_DB_CONNECTIONS)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
        .map_err(|e| {
            let err = AppError::DatabaseError(e);
            error!(error = %err, "Failed to initialize pool");
            err
        })
}

/// Verifies the connection is alive by running a simple query.
#[instrument(skip(pool))]
pub async fn check_connection(pool: &PgPool) -> Result<(), AppError> {
    info!("Checking database connection");

    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map(|_| {
            info!("Database connection is healthy");
        })
        .map_err(|e| {
            let err = AppError::DatabaseError(e);
            error!(error = %err, "Health check failed");
            err
        })?;

    Ok(())
}

/// Runs pending migrations from the ./migrations directory.
#[instrument(skip(pool))]
pub async fn run_migrations(pool: &PgPool) -> Result<(), AppError> {
    info!("Running database migrations");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| {
            let err = AppError::MigrationError(e.to_string());
            error!(error = %err, "Migration execution failed");
            err
        })?;

    info!("Database migrations completed successfully");
    Ok(())
}
