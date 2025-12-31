use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct NearbyStationsQuery {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default = "default_radius")]
    pub radius_meters: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_radius() -> i32 {
    5000
}

fn default_limit() -> i32 {
    50
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StationResponse {
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReviewRequest {
    pub station_id: String,
    pub rating: i32,
    pub review_text: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateReviewRequest {
    pub rating: Option<i32>,
    pub review_text: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReviewResponse {
    pub review_id: String,
    pub user_id: Option<String>,
    pub station_id: String,
    pub rating: Option<i32>,
    pub review_text: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
