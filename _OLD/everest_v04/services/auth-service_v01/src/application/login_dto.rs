use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String, // Can be email or username
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LoginMetadata>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginMetadata {
    pub login_ip: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}