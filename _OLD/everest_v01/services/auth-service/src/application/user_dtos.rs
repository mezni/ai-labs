// src/application/user_dtos.rs
use crate::domain::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// User response DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub is_verified: bool,
    pub role: String,
    pub network_id: String,
    pub station_id: String,
    pub source: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            email: user.email.into(),
            username: user.username.as_str().to_string(),
            first_name: user.name.first_name().map(|s| s.to_string()),
            last_name: user.name.last_name().map(|s| s.to_string()),
            phone: user.phone.into_inner(),
            photo: user.photo,
            is_verified: user.is_verified,
            role: user.role.into(),
            network_id: user.network_id,
            station_id: user.station_id,
            source: user.source.into(),
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Create internal user request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateInternalUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 100, message = "Username must be 3-100 characters"))]
    pub username: String,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(custom(function = "validate_role"))]
    pub role: String,

    #[serde(default)]
    pub network_id: Option<String>,

    #[serde(default)]
    pub station_id: Option<String>,
}

fn validate_role(role: &String) -> Result<(), validator::ValidationError> {
    if crate::core::constants::is_valid_role(role) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_role"))
    }
}

/// Update user profile request (self-update)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo: Option<String>,
}

/// Admin update user request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct AdminUpdateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(length(min = 3, max = 100))]
    pub username: Option<String>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo: Option<String>,

    #[validate(custom(function = "validate_role"))]
    pub role: String,

    pub network_id: Option<String>,
    pub station_id: Option<String>,
}

/// List users request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ListUsersRequest {
    pub search: Option<String>,
    pub role: Option<String>,
    pub source: Option<String>,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,

    #[serde(default)]
    pub include_deleted: bool,

    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: i64,

    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_page_size")]
    pub page_size: i64,

    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    20
}

/// Paginated user list response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedUsersResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

/// User detail response (includes more info)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserDetailResponse {
    #[serde(flatten)]
    pub user: UserResponse,
    pub keycloak_id: String,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl UserDetailResponse {
    pub fn from_user(user: User) -> Self {
        Self {
            keycloak_id: user.keycloak_id.clone(),
            created_by: user.created_by.clone(),
            updated_by: user.updated_by.clone(),
            user: UserResponse::from(user),
        }
    }
}

/// Delete user response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeleteUserResponse {
    pub message: String,
    pub user_id: String,
    pub deleted_at: DateTime<Utc>,
}

/// Reset password request (admin-triggered)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct AdminResetPasswordRequest {
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,

    #[serde(default)]
    pub temporary: bool,
}

/// Generic success response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SuccessResponse {
    pub message: String,
}

/// Error response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
