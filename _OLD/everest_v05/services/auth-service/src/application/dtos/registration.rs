use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct RegisterUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(max = 50))]
    pub first_name: Option<String>,

    #[validate(length(max = 50))]
    pub last_name: Option<String>,

    #[validate(length(max = 20))]
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RegisterUserResponse {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct VerifyUserRequest {
    #[validate(email)]
    pub email: String,

    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct ResendVerificationRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct VerifyUserResponse {
    pub status: String,
    pub message: String,
}
