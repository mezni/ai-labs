use sqlx::{migrate::Migrator, PgPool};

use super::error::DatabaseError;

// Embed migration files into the compiled binary.
static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

/// Applies all pending database migrations.
pub async fn run_migrations(
    pool: &PgPool,
) -> Result<(), DatabaseError> {
    MIGRATOR
        .run(pool)
        .await
        .map_err(DatabaseError::Migration)?;

    Ok(())
}