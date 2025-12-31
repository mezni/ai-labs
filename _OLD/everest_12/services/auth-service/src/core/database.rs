use crate::core::{
    constants::{MAX_DB_CONNECTIONS, MIN_DB_CONNECTIONS},
    errors::AppError,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub type DbPool = PgPool;

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .min_connections(MIN_DB_CONNECTIONS)
        .connect(database_url)
        .await
        .map_err(AppError::DatabaseError)?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(AppError::MigrationError)?;

    Ok(pool)
}
