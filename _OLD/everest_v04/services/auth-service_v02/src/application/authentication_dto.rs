use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub metadata: Option<AuthMetadata>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AuthMetadata {
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyMetadata {
    pub verification_ip: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyRequest {
    pub token: String,
    pub metadata: Option<VerifyMetadata>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyResponse {
    pub user_id: String,
    pub email: String,
    pub message: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}
