use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::{info, error};

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    info!("Creating database connection pool");
    
    let pool = PgPoolOptions::new()
        .max_connections(crate::core::constants::MAX_DB_CONNECTIONS)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
        .map_err(|e| {
            error!("Failed to create database pool: {}", e);
            anyhow::anyhow!("Database connection failed: {}", e)
        })?;
    
    info!("Database connection pool created successfully");
    Ok(pool)
}

pub async fn check_connection(pool: &PgPool) -> anyhow::Result<()> {
    info!("Checking database connection");
    
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("Database health check failed: {}", e);
            anyhow::anyhow!("Database health check failed: {}", e)
        })?;
    
    info!("Database connection is healthy");
    Ok(())
}

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    info!("Running database migrations");
    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| {
            error!("Failed to run migrations: {}", e);
            anyhow::anyhow!("Migration failed: {}", e)
        })?;
    
    info!("Database migrations completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // Run only when database is available
    async fn test_create_pool() {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/test_db".to_string());
        
        let pool = create_pool(&database_url).await;
        assert!(pool.is_ok());
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_check_connection() {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/test_db".to_string());
        
        let pool = create_pool(&database_url).await.unwrap();
        let result = check_connection(&pool).await;
        assert!(result.is_ok());
    }
}