use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub version: String,
    pub uptime_seconds: u64,
    pub timestamp: DateTime<Utc>,
    pub checks: HealthChecks,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthChecks {
    pub database: DatabaseHealth,
    pub keycloak: ServiceHealth,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatabaseHealth {
    pub status: ComponentStatus,
    pub connected: bool,
    pub response_time_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub details: DatabaseDetails,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatabaseDetails {
    pub pool_size: u32,
    pub idle_connections: u32,
    pub active_connections: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServiceHealth {
    pub status: ComponentStatus,
    pub available: bool,
    pub response_time_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComponentStatus {
    Up,
    Down,
    Degraded,
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self {
            status: HealthStatus::Healthy,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: 0,
            timestamp: Utc::now(),
            checks: HealthChecks {
                database: DatabaseHealth {
                    status: ComponentStatus::Down,
                    connected: false,
                    response_time_ms: None,
                    error: None,
                    details: DatabaseDetails {
                        pool_size: 0,
                        idle_connections: 0,
                        active_connections: 0,
                    },
                },
                keycloak: ServiceHealth {
                    status: ComponentStatus::Down,
                    available: false,
                    response_time_ms: None,
                    error: None,
                },
            },
        }
    }
}

