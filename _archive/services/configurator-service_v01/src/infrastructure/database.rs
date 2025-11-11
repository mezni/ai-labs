use shared::config::AppConfig;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(config: &AppConfig) -> Result<Self, sqlx::Error> {
        info!("Connecting to database...");

        let pool = PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .acquire_timeout(Duration::from_secs(config.database.timeout_seconds))
            .connect(&config.database.url)
            .await?;

        info!("Database connection established successfully");

        Ok(Self { pool })
    }

    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
