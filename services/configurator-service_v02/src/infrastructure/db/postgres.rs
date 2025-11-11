use sqlx::{PgPool, Pool, Postgres};
use crate::core::config::AppConfig;
use crate::core::errors::ServiceError;

#[derive(Clone)]
pub struct PostgresManager {
    pub pool: PgPool,
}

impl PostgresManager {
    pub async fn new(config: &AppConfig) -> Result<Self, ServiceError> {
        let database_url = config.get_database_url()?;
        let pool = PgPool::connect(&database_url)
            .await
            .map_err(|e| ServiceError::internal(&format!("Failed to connect to Postgres: {}", e)))?;
        Ok(Self { pool })
    }
}
