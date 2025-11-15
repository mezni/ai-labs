use crate::core::constants::DEFAULT_MAX_CONNECTIONS;
use crate::core::errors::AppError;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};
use utoipa::ToSchema;

#[derive(Clone)]
pub struct ConnectionManager {
    pool: Arc<PgPool>,
}

impl ConnectionManager {
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        Self::with_config(database_url, DEFAULT_MAX_CONNECTIONS).await
    }

    pub async fn with_config(database_url: &str, max_connections: u32) -> Result<Self, AppError> {
        info!(
            "Initializing connection pool with {} max connections",
            max_connections
        );

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await
            .map_err(|e| {
                error!("Failed to create connection pool: {}", e);
                AppError::PoolError(e.to_string())
            })?;

        // Test the connection
        sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| {
            error!("Failed to test database connection: {}", e);
            AppError::PoolError(e.to_string())
        })?;

        info!("Connection pool initialized successfully");

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn health_check(&self) -> Result<(), AppError> {
        sqlx::query("SELECT 1")
            .execute(self.get_pool())
            .await
            .map_err(|e| {
                error!("Health check failed: {}", e);
                AppError::PoolError(e.to_string())
            })?;
        Ok(())
    }

    pub async fn get_connection_stats(&self) -> Result<PoolStats, AppError> {
        let size = self.pool.size();
        let num_idle = self.pool.num_idle();
        let num_used = size as u32 - num_idle as u32;

        Ok(PoolStats {
            total_connections: size as u32,
            idle_connections: num_idle as u32,
            used_connections: num_used,
            max_connections: self.pool.options().get_max_connections(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PoolStats {
    pub total_connections: u32,
    pub idle_connections: u32,
    pub used_connections: u32,
    pub max_connections: u32,
}
