use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 100))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,

    pub first_name: Option<String>,
    pub last_name: Option<String>,

    #[validate(length(min = 10, max = 20))]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub registration_id: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VerifyRequest {
    #[validate(email)]
    pub email: String,

    /// The token sent to the user's email
    #[validate(length(min = 1))]
    pub token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResendRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResendResponse {
    pub message: String,
}
