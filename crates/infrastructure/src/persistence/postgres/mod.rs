use sqlx::postgres::PgPool;
use sqlx::Pool;

pub type PgPool = Pool<sqlx::Postgres>;

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::new(database_url);
    pool.acquire().await.ok();
    Ok(pool)
}