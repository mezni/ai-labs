use crate::AppState;
use crate::core::errors::AppResult;
use std::sync::Arc;

pub struct HealthService {
    state: Arc<AppState>,
}

impl HealthService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn check(&self) -> AppResult<HealthStatus> {
        // Fixes E0609: Use self.state.db_pool instead of self.pool
        let db_healthy = sqlx::query("SELECT 1")
            .fetch_one(&self.state.db_pool)
            .await
            .is_ok();

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
