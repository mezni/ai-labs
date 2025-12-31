use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateInvitationRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub role: Option<String>,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InvitationResponse {
    pub code: String,
    pub email: String,
    pub role: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InvitationDetailResponse {
    pub id: Uuid,
    pub code: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub created_by: Uuid,
    pub accepted_by: Option<Uuid>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AcceptInvitationRequest {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}