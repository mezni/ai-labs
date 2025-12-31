use crate::core::errors::AppError;
use sqlx::{PgPool, postgres::PgPoolOptions};
pub type DbPool = PgPool;

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(crate::core::constants::MAX_DB_CONNECTIONS)
        .connect(database_url)
        .await
        .map_err(AppError::DatabaseError)?;

    // Automatically run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| AppError::Internal(format!("Migration failed: {}", e)))?;

    Ok(pool)
}
