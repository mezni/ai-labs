use crate::core::errors::AppResult;
use sqlx::PgPool;

pub struct HealthService {
    pool: PgPool,
}

impl HealthService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn check(&self) -> AppResult<HealthStatus> {
        let db_healthy = sqlx::query("SELECT 1").fetch_one(&self.pool).await.is_ok();

        Ok(HealthStatus {
            status: if db_healthy { "healthy" } else { "unhealthy" }.to_string(),
            database: db_healthy,
        })
    }
}

#[derive(serde::Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub database: bool,
}
