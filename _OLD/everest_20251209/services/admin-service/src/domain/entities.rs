use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum NetworkType {
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    #[serde(rename = "COMPANY")]
    Company,
}

impl NetworkType {
    pub fn as_str(&self) -> &str {
        match self {
            NetworkType::Individual => "INDIVIDUAL",
            NetworkType::Company => "COMPANY",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Network {
    pub network_id: String,
    pub name: String,
    pub network_type: String,
    pub support_phone: Option<String>,
    pub support_email: Option<String>,
    pub is_verified: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Station {
    pub station_id: String,
    pub osm_id: i64,
    pub name: String,
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    pub tags: Option<serde_json::Value>,
    pub network_id: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Connector {
    pub connector_id: String,
    pub station_id: String,
    pub connector_type_id: i64,
    pub status_id: i64,
    pub current_type_id: i64,
    #[schema(value_type = String)]
    pub power_kw: Option<bigdecimal::BigDecimal>,
    pub voltage: Option<i32>,
    pub amperage: Option<i32>,
    pub count_available: Option<i32>,
    pub count_total: Option<i32>,
    pub created_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}
