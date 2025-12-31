use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInvitationRequest {
    #[validate(email)]
    pub email: String,

    #[serde(default = "default_role")]
    pub role: String,

    #[serde(default = "default_expires_in")]
    pub expires_in_hours: i64,

    pub metadata: Option<serde_json::Value>,
}

fn default_role() -> String {
    "user".to_string()
}

fn default_expires_in() -> i64 {
    72 // 3 days
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InvitationResponse {
    pub invitation_id: String,
    pub code: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub expires_at: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InvitationListResponse {
    pub invitations: Vec<InvitationResponse>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AcceptInvitationRequest {
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcceptInvitationResponse {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub message: String,
}
