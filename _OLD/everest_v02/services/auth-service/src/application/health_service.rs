use crate::core::constants::APP_VERSION;
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthReport {
    pub status: String,
    pub db_connected: bool,
    pub version: String,
}

pub struct HealthService;

impl HealthService {
    /// Returns the system health status, including DB connectivity.
    pub async fn check_system_status(db_pool: &PgPool) -> HealthReport {
        let db_ok = sqlx::query("SELECT 1").execute(db_pool).await.is_ok();

        HealthReport {
            status: match db_ok {
                true => "UP",
                false => "DOWN",
            }
            .to_string(),
            db_connected: db_ok,
            version: APP_VERSION.to_string(),
        }
    }
}
