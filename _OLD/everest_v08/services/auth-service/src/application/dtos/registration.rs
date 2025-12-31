use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::enums::{RegistrationStatus, Source};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "user@example.com")]
    pub email: String,

    #[schema(example = "johndoe")]
    pub username: String,

    #[schema(example = "SecurePassword123!")]
    pub password: String,

    #[schema(example = "John")]
    pub first_name: Option<String>,

    #[schema(example = "Doe")]
    pub last_name: Option<String>,

    #[schema(example = "+1234567890")]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    pub registration_id: String,
    pub email: String,
    pub username: String,
    pub status: RegistrationStatus,
    pub expires_at: DateTime<Utc>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyRequest {
    #[schema(example = "user@example.com")]
    pub email: String,

    #[schema(example = "123456")]
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyResponse {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResendVerificationRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResendVerificationResponse {
    pub message: String,
    pub expires_at: DateTime<Utc>,
}
