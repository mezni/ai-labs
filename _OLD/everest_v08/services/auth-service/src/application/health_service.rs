use crate::application::dtos::health::HealthResponse;
use crate::core::constants::APP_VERSION;
use sqlx::PgPool;
use tracing::{instrument, warn};

pub struct HealthService;

impl HealthService {
    /// Checks the health of the application and its dependencies.
    /// Uses #[instrument] to link logs to this specific check.
    #[instrument(skip(db_pool))]
    pub async fn check_health(db_pool: &PgPool) -> HealthResponse {
        let mut overall_status = "ok";

        // Check database connectivity
        let db_status = match sqlx::query("SELECT 1").fetch_one(db_pool).await {
            Ok(_) => "up".to_string(),
            Err(e) => {
                // We log the actual error here for debugging
                warn!(error = %e, "Database health check failed");
                overall_status = "error";
                "down".to_string()
            }
        };

        HealthResponse {
            status: overall_status.to_string(),
            version: APP_VERSION.to_string(),
            database: db_status,
        }
    }
}
