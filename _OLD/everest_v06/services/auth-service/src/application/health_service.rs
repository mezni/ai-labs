use crate::application::dtos::health::HealthResponse;
use crate::core::constants::APP_VERSION;
use crate::core::database::DbPool;

pub struct HealthService;

impl HealthService {
    pub async fn check_health(db_pool: &DbPool) -> HealthResponse {
        let db_status = match sqlx::query("SELECT 1").execute(db_pool).await {
            Ok(_) => "up".to_string(),
            Err(_) => "down".to_string(),
        };

        HealthResponse {
            status: "ok".to_string(),
            version: APP_VERSION.to_string(),
            database: db_status,
        }
    }
}
