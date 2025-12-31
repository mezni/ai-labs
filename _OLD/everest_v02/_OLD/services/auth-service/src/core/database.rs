use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn init_db_pool(database_url: &str, max_connections: u32) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;

    tracing::info!("Database connection pool initialized");
    Ok(pool)
}
