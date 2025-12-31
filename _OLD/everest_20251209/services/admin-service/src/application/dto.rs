use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Network DTOs
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNetworkRequest {
    pub name: String,
    pub network_type: String, // "INDIVIDUAL" or "COMPANY"
    pub support_phone: Option<String>,
    pub support_email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateNetworkRequest {
    pub name: Option<String>,
    pub network_type: Option<String>,
    pub support_phone: Option<String>,
    pub support_email: Option<String>,
    pub is_verified: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NetworkResponse {
    pub network_id: String,
    pub name: String,
    pub network_type: String,
    pub support_phone: Option<String>,
    pub support_email: Option<String>,
    pub is_verified: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// Station DTOs
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateStationRequest {
    pub osm_id: i64,
    pub name: String,
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub tags: Option<serde_json::Value>,
    pub network_id: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateStationRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub tags: Option<serde_json::Value>,
    pub network_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StationResponse {
    pub station_id: String,
    pub osm_id: i64,
    pub name: String,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub tags: Option<serde_json::Value>,
    pub network_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// Connector DTOs
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateConnectorRequest {
    pub station_id: String,
    pub connector_type_id: i64,
    pub status_id: i64,
    pub current_type_id: i64,
    pub power_kw: Option<f64>,
    pub voltage: Option<i32>,
    pub amperage: Option<i32>,
    pub count_available: Option<i32>,
    pub count_total: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateConnectorRequest {
    pub status_id: Option<i64>,
    pub power_kw: Option<f64>,
    pub voltage: Option<i32>,
    pub amperage: Option<i32>,
    pub count_available: Option<i32>,
    pub count_total: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConnectorResponse {
    pub connector_id: String,
    pub station_id: String,
    pub connector_type_id: i64,
    pub status_id: i64,
    pub current_type_id: i64,
    pub power_kw: Option<String>,
    pub voltage: Option<i32>,
    pub amperage: Option<i32>,
    pub count_available: i32,
    pub count_total: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    50
}
