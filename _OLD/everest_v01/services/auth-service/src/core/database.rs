// src/core/database.rs
use crate::core::{config::DatabaseConfig, errors::AppResult};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(config: &DatabaseConfig) -> AppResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&config.url)
        .await?;

    tracing::info!(
        "Database pool created with {} max connections",
        config.max_connections
    );

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> AppResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}

// Helper for transaction management
pub struct DbTransaction<'a> {
    tx: sqlx::Transaction<'a, sqlx::Postgres>,
}

impl<'a> DbTransaction<'a> {
    pub async fn begin(pool: &'a PgPool) -> AppResult<Self> {
        let tx = pool.begin().await?;
        Ok(Self { tx })
    }

    pub async fn commit(self) -> AppResult<()> {
        self.tx.commit().await?;
        Ok(())
    }

    pub async fn rollback(self) -> AppResult<()> {
        self.tx.rollback().await?;
        Ok(())
    }

    pub fn as_mut(&mut self) -> &mut sqlx::Transaction<'a, sqlx::Postgres> {
        &mut self.tx
    }
}
