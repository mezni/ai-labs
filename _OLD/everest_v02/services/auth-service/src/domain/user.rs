use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub keycloak_id: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub is_verified: bool,
    pub role: UserRole,
    pub source: UserSource,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum UserRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "operator")]
    Operator,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::Partner => write!(f, "partner"),
            UserRole::Operator => write!(f, "operator"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum UserSource {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "internal")]
    Internal,
}

impl std::fmt::Display for UserSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSource::Web => write!(f, "web"),
            UserSource::Internal => write!(f, "internal"),
        }
    }
}

impl User {
    pub fn new(
        user_id: String,
        keycloak_id: String,
        email: String,
        username: String,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
    ) -> Self {
        Self {
            user_id,
            keycloak_id,
            email,
            username,
            first_name,
            last_name,
            phone,
            photo: None,
            is_verified: true,
            role: UserRole::User,
            source: UserSource::Web,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
