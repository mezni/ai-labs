use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::core::constants::DEFAULT_INVITATION_EXPIRES_HOURS;

/// Request to create a new invitation
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInvitationRequest {
    #[validate(email)]
    pub email: String,

    pub role: String, // no default

    #[serde(default = "default_expires_in")]
    pub expires_in_hours: i64, // default from constants

    pub metadata: Option<serde_json::Value>,
}

fn default_expires_in() -> i64 {
    DEFAULT_INVITATION_EXPIRES_HOURS
}

/// Response for a single invitation
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

/// Response for paginated invitations
#[derive(Debug, Serialize, ToSchema)]
pub struct InvitationListResponse {
    pub invitations: Vec<InvitationResponse>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

/// Request to accept an invitation
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AcceptInvitationRequest {
    #[validate(length(min = 8))]
    pub password: String,
}

/// Response after accepting an invitation
#[derive(Debug, Serialize, ToSchema)]
pub struct AcceptInvitationResponse {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub message: String,
}
