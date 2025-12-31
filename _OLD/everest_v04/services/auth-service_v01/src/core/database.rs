use sqlx::postgres::{PgPool, PgPoolOptions};

pub type DbPool = PgPool;

pub async fn create_pool(database_url: &str) -> Result<DbPool, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(database_url)
        .await?;
    
    // Test connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;
    
    tracing::info!("Database connection pool established");
    
    Ok(pool)
}