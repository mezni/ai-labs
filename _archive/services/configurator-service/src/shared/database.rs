use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::shared::config::AppConfig;

/// Create PostgreSQL connection pool
pub async fn create_pg_pool(config: &AppConfig) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;
    Ok(pool)
}

/// Wrap PgPool for Actix injection
pub fn pool_data(pool: PgPool) -> actix_web::web::Data<PgPool> {
    actix_web::web::Data::new(pool)
}
