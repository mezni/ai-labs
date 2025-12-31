use sqlx::PgPool;

use crate::application::dtos::health::HealthResponse;
use crate::core::errors::AppResult;

pub struct HealthService {
    pool: PgPool,
}

impl HealthService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn check_health(&self) -> AppResult<HealthResponse> {
        let db_status = match sqlx::query("SELECT 1").execute(&self.pool).await {
            Ok(_) => "connected",
            Err(_) => "disconnected",
        };

        Ok(HealthResponse {
            status: if db_status == "connected" {
                "healthy".to_string()
            } else {
                "unhealthy".to_string()
            },
            database: db_status.to_string(),
            timestamp: chrono::Utc::now(),
        })
    }
}