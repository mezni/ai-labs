use std::sync::Arc;
use std::time::{Duration, Instant};
use sqlx::PgPool;
use tracing::{debug, error, warn};

use crate::core::errors::AppError;
use crate::application::dtos::health_dto::{
    HealthResponse, HealthStatus, HealthChecks, DatabaseHealth, ServiceHealth,
    ComponentStatus, DatabaseDetails,
};

pub struct HealthService {
    pool: PgPool,
    keycloak_url: String,
    start_time: Instant,
}

impl HealthService {
    pub fn new(pool: PgPool, keycloak_url: String) -> Self {
        Self {
            pool,
            keycloak_url,
            start_time: Instant::now(),
        }
    }

    pub async fn check_health(&self) -> Result<HealthResponse, AppError> {
        debug!("Performing health check");
        
        let timestamp = chrono::Utc::now();
        let uptime_seconds = self.start_time.elapsed().as_secs();
        
        // Check database health
        let database = self.check_database().await;
        
        // Check Keycloak health
        let keycloak = self.check_keycloak().await;
        
        // Determine overall health status
        let status = self.determine_overall_status(&database, &keycloak);
        
        let response = HealthResponse {
            status,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds,
            timestamp,
            checks: HealthChecks {
                database,
                keycloak,
            },
        };
        
        debug!("Health check completed with status: {:?}", response.status);
        Ok(response)
    }

    async fn check_database(&self) -> DatabaseHealth {
        let start = Instant::now();
        
        match self.perform_database_check().await {
            Ok(details) => {
                let response_time_ms = start.elapsed().as_millis() as u64;
                debug!("Database health check passed in {}ms", response_time_ms);
                
                DatabaseHealth {
                    status: ComponentStatus::Up,
                    connected: true,
                    response_time_ms: Some(response_time_ms),
                    error: None,
                    details,
                }
            }
            Err(e) => {
                error!("Database health check failed: {}", e);
                
                DatabaseHealth {
                    status: ComponentStatus::Down,
                    connected: false,
                    response_time_ms: None,
                    error: Some(e.to_string()),
                    details: DatabaseDetails {
                        pool_size: 0,
                        idle_connections: 0,
                        active_connections: 0,
                    },
                }
            }
        }
    }

    async fn perform_database_check(&self) -> Result<DatabaseDetails, AppError> {
        // Perform a simple query to check database connectivity
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        // Get pool statistics
        let pool_size = self.pool.size();
        let idle_connections = self.pool.num_idle();
        
        Ok(DatabaseDetails {
            pool_size,
            idle_connections,
            active_connections: pool_size - idle_connections,
        })
    }

    async fn check_keycloak(&self) -> ServiceHealth {
        let start = Instant::now();
        
        match self.perform_keycloak_check().await {
            Ok(_) => {
                let response_time_ms = start.elapsed().as_millis() as u64;
                debug!("Keycloak health check passed in {}ms", response_time_ms);
                
                ServiceHealth {
                    status: ComponentStatus::Up,
                    available: true,
                    response_time_ms: Some(response_time_ms),
                    error: None,
                }
            }
            Err(e) => {
                warn!("Keycloak health check failed: {}", e);
                
                ServiceHealth {
                    status: ComponentStatus::Down,
                    available: false,
                    response_time_ms: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    async fn perform_keycloak_check(&self) -> Result<(), AppError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        
        let url = format!("{}/health", self.keycloak_url);
        
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Failed to connect: {}", e)))?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(AppError::KeycloakError(format!(
                "Unhealthy status: {}",
                response.status()
            )))
        }
    }

    fn determine_overall_status(
        &self,
        database: &DatabaseHealth,
        keycloak: &ServiceHealth,
    ) -> HealthStatus {
        // Critical: Database must be up
        if database.status == ComponentStatus::Down {
            return HealthStatus::Unhealthy;
        }
        
        // Non-critical: Keycloak can be down (degraded mode)
        if keycloak.status == ComponentStatus::Down {
            return HealthStatus::Degraded;
        }
        
        // Both services are up
        HealthStatus::Healthy
    }
}

