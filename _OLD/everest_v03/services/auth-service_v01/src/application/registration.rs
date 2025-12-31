use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RegistrationRequest {
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "Username must be between 3 and 100 characters"
    ))]
    #[schema(example = "johndoe", min_length = 3, max_length = 100)]
    pub username: String,

    #[schema(example = "John", nullable = true)]
    pub first_name: Option<String>,

    #[schema(example = "Doe", nullable = true)]
    pub last_name: Option<String>,

    #[schema(example = "+15551234567", nullable = true)]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResendRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationStatusResponse {
    pub id: String,
    pub status: String,
    pub verified: bool,
    pub created_at: String,
}
