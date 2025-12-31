use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct RegisterUserRequest {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterUserResponse {
    pub registration_id: String,
    pub status: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyRegistrationRequest {
    #[schema(example = "REG-TFCWVPyqoGxHljnW")]
    pub registration_id: String,

    #[schema(example = "9999")]
    pub token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyRegistrationResponse {
    pub registration_id: String,
    pub status: String,
    pub message: String,
}
