use crate::core::errors::AppError;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(crate::core::constants::MAX_DB_CONNECTIONS)
        .connect(database_url)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Automatically run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Migration failed: {}", e)))?;

    Ok(pool)
}
