use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Station {
    pub station_id: String,
    pub name: String,
    pub address: Option<String>,
    pub distance_meters: Option<f64>,
    pub has_available_connectors: Option<bool>,
    pub total_available_connectors: Option<i64>,
    pub max_power_kw: Option<f64>,
    pub power_tier: Option<String>,
    pub operator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct UserReview {
    pub review_id: String,
    pub user_id: Option<String>,
    pub station_id: String,
    pub rating: Option<i32>,
    pub review_text: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
