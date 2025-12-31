use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
    Partner,
    Operator,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::Admin => "admin",
            UserRole::Partner => "partner",
            UserRole::Operator => "operator",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "user" => Some(UserRole::User),
            "admin" => Some(UserRole::Admin),
            "partner" => Some(UserRole::Partner),
            "operator" => Some(UserRole::Operator),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserSource {
    Web,
    Internal,
}

impl UserSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserSource::Web => "web",
            UserSource::Internal => "internal",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
    pub role: String,
    pub network_id: String,
    pub station_id: String,
    pub source: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
    pub refresh_expires_in: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakUser {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub enabled: bool,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    pub attributes: Option<KeycloakUserAttributes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakUserAttributes {
    pub network_id: Option<Vec<String>>,
    pub station_id: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
    pub refresh_expires_in: Option<i32>,
}
