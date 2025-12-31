use crate::AppState;
use crate::application::dtos::health::{ComponentStatus, HealthDetails, HealthStatus};
use crate::core::errors::AppResult;
use std::sync::Arc;
use std::time::Duration;

pub struct HealthService {
    state: Arc<AppState>,
}

impl HealthService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn check(&self) -> AppResult<(HealthStatus, HealthDetails)> {
        let db_healthy = sqlx::query("SELECT 1")
            .fetch_one(&self.state.db_pool)
            .await
            .is_ok();

        let keycloak_healthy = self.check_keycloak().await;

        let overall_status = if db_healthy && keycloak_healthy {
            HealthStatus::Up
        } else {
            HealthStatus::Down
        };

        Ok((
            overall_status,
            HealthDetails {
                database: if db_healthy {
                    ComponentStatus::Up
                } else {
                    ComponentStatus::Down
                },
                keycloak: if keycloak_healthy {
                    ComponentStatus::Up
                } else {
                    ComponentStatus::Down
                },
            },
        ))
    }

    async fn check_keycloak(&self) -> bool {
        let url = format!(
            "{}/realms/{}/.well-known/openid-configuration",
            self.state.config.keycloak_url, self.state.config.keycloak_realm
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build();

        let client = match client {
            Ok(c) => c,
            Err(_) => return false,
        };

        client
            .get(url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}
