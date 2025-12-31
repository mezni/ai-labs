use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct Metadata {
    pub registration_ip: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    //    #[serde(default = "default_source")]
    //    pub source: String,
    //    #[serde(skip_serializing_if = "Option::is_none")]
    //    pub metadata: Option<Metadata>,
}

fn default_source() -> String {
    "web".to_string()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub registration_id: String,
    pub email: String,
    pub expires_at: DateTime<Utc>,
    pub message: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ResendVerificationRequest {
    pub email: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResendVerificationResponse {
    pub message: String,
    pub expires_at: DateTime<Utc>,
}
