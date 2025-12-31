use crate::domain::entities::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub is_verified: bool,
    pub role: String,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 100))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,

    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "user".to_string()
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 3, max = 100))]
    pub username: Option<String>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            is_verified: user.is_verified,
            // Use lowercase to match Keycloak expectations
            role: format!("{:?}", user.role).to_lowercase(),
            is_active: user.is_active,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}
