use crate::core::constants::{MAX_DB_CONNECTIONS, MIN_DB_CONNECTIONS};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Creates and configures the database connection pool using defined constants.
pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .min_connections(MIN_DB_CONNECTIONS)
        .max_connections(MAX_DB_CONNECTIONS)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    // Run migrations as part of initialization
    run_migrations(&pool).await?;

    Ok(pool)
}

/// Dedicated function to handle database migrations.
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    tracing::info!("Running database migrations...");

    sqlx::migrate!("./migrations").run(pool).await?;

    tracing::info!("Migrations completed successfully");
    Ok(())
}
