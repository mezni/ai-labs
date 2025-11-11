use crate::core::config::AppConfig;
use crate::core::errors::AppError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

/// Database connection pool wrapper
#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Creates a new database connection pool
    pub async fn new(config: &AppConfig) -> Result<Self, AppError> {
        let database_url = config.get_database_url()?;
        let max_connections = config.get_max_db_connections();

        info!("Connecting to database with {max_connections} max connections...");

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800))
            .connect(&database_url)
            .await
            .map_err(|e| AppError::database(format!("Failed to connect to database: {e}")))?;

        // Test connection
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .map_err(|e| AppError::database(format!("Database connection test failed: {e}")))?;

        info!("Database connection established successfully");

        Ok(Self { pool })
    }

    /// Returns a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Performs a health check on the database connection
    pub async fn health_check(&self) -> bool {
        sqlx::query("SELECT 1").execute(&self.pool).await.is_ok()
    }
}
