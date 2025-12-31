// src/application/auth_dtos.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Registration request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 100, message = "Username must be 3-100 characters"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

/// Registration response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    pub registration_id: String,
    pub email: String,
    pub message: String,
    pub expires_at: DateTime<Utc>,
}

/// Email verification request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct VerifyEmailRequest {
    #[validate(length(min = 1))]
    pub token: String,
}

/// Email verification response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VerifyEmailResponse {
    pub message: String,
    pub user_id: String,
    pub email: String,
}

/// Login request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username or email is required"))]
    pub username: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// Login response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub network_id: String,
    pub station_id: String,
}

/// Refresh token request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

/// Change password request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1))]
    pub old_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

/// Request password reset
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RequestPasswordResetRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Password reset request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1))]
    pub token: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
}

/// Logout request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

/// Token response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}

/// Generic auth success response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthSuccessResponse {
    pub message: String,
}
