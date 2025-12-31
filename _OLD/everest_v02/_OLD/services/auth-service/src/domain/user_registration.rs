use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistration {
    pub registration_id: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub verification_token: String,
    pub status: RegistrationStatus,
    pub keycloak_id: Option<String>,
    pub user_id: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text")]
pub enum RegistrationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl std::fmt::Display for RegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationStatus::Pending => write!(f, "pending"),
            RegistrationStatus::Verified => write!(f, "verified"),
            RegistrationStatus::Expired => write!(f, "expired"),
            RegistrationStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl UserRegistration {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn can_verify(&self) -> bool {
        self.status == RegistrationStatus::Pending && !self.is_expired()
    }
}
