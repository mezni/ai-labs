use crate::domain::UserRole;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct RegisterRequest {
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

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateUserRequest {
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

    pub photo: Option<String>,

    pub role: UserRole,

    #[validate(length(max = 32))]
    pub network_id: Option<String>,

    #[validate(length(max = 32))]
    pub station_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub username: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1))]
    pub old_password: String,

    #[validate(length(min = 8))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(max = 50))]
    pub first_name: Option<String>,

    #[validate(length(max = 50))]
    pub last_name: Option<String>,

    #[validate(length(max = 20))]
    pub phone: Option<String>,

    pub photo: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user_id: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<crate::domain::User>,
    pub total: usize,
}
