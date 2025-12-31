use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyMetadata {
    pub verification_ip: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyRequest {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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