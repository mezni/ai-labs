use super::enums::{RegistrationStatus, Source, UserRole};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
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

    pub network_id: Option<String>,
    pub station_id: Option<String>,
    pub source: Source,

    pub is_active: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRegistration {
    pub registration_id: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub verification_token: String,
    pub status: RegistrationStatus,
    // FIX: keycloak_id is nullable in your SQL schema
    pub keycloak_id: Option<String>,
    pub user_id: Option<String>,
    // SQL default is 0, but field is technically NOT NULL (implicit)
    pub resend_count: i32,
    pub expires_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub source: Source,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Invitation {
    pub invitation_id: String,
    pub code: String,
    pub email: String,
    pub role: super::enums::UserRole,
    pub invited_by: String,
    pub status: super::enums::InvitationStatus,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub accepted_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RefreshToken {
    pub token_id: String,
    pub user_id: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}
