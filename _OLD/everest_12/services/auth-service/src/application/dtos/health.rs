use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub details: Option<HealthDetails>,
}

#[derive(Debug, Serialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Up,
    Down,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthDetails {
    pub database: ComponentStatus,
    pub keycloak: ComponentStatus,
}

#[derive(Debug, Serialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ComponentStatus {
    Up,
    Down,
}
