use crate::domain::entities::{Role, User, UserSource};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user_id: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInternalUserRequest {
    #[validate(email)]
    pub email: String,
    pub roles: Vec<String>,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    pub roles: Option<Vec<String>>,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateMeRequest {
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub source: String,
    pub roles: Vec<String>,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            source: match user.source {
                UserSource::Web => "web".to_string(),
                UserSource::Internal => "internal".to_string(),
            },
            roles: user.roles.iter().map(|r| r.as_str().to_string()).collect(),
            network_id: user.network_id,
            station_id: user.station_id,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListUsersResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
}