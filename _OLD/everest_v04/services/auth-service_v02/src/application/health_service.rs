use crate::AppState;
use crate::application::health_dto::HealthResponseDto;
use crate::infrastructure::keycloak_client::KeycloakClient;

pub struct HealthService;

impl HealthService {
    pub async fn check_health(state: &AppState) -> HealthResponseDto {
        // 1. Check Database using .db_pool
        let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db_pool).await {
            Ok(_) => "up".to_string(),
            Err(_) => "down".to_string(),
        };

        // 2. Check Keycloak using .keycloak_client
        // Note: Use a method that actually exists on your client, like 'ping' or 'get_status'
        let keycloak_status = match state.keycloak_client.get_user_info("ping").await {
            Ok(_) => "up".to_string(),
            Err(_) => "down".to_string(),
        };

        let overall_status = if db_status == "up" && keycloak_status == "up" {
            "ok"
        } else {
            "unhealthy"
        };

        HealthResponseDto {
            status: overall_status.to_string(),
            database: db_status.to_string(),
            keycloak: keycloak_status.to_string(),
        }
    }
}
