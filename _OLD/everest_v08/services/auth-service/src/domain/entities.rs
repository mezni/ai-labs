use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::domain::enums::{Role, RegistrationStatus, Source, InvitationStatus};

/// User entity - represents a verified user account synchronized with Keycloak
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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
    pub role: Role,
    
    pub network_id: String,
    pub station_id: String,
    pub source: Source,
    
    pub is_active: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl User {
    pub fn new(
        user_id: String,
        keycloak_id: String,
        email: String,
        username: String,
        role: Role,
        source: Source,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            keycloak_id,
            email,
            username,
            first_name: None,
            last_name: None,
            phone: None,
            photo: None,
            is_verified: true,
            role,
            network_id: String::new(),
            station_id: String::new(),
            source,
            is_active: true,
            deleted_at: None,
            last_login_at: None,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }

    pub fn full_name(&self) -> Option<String> {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            (None, None) => None,
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn soft_delete(&mut self) {
        self.is_active = false;
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn update_last_login(&mut self) {
        self.last_login_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

/// User Registration entity - temporary storage for pending registrations
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Registration {
    pub registration_id: String,
    
    pub email: String,
    pub username: String,
    
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    
    pub verification_token: String,
    pub status: RegistrationStatus,
    
    pub keycloak_id: String,
    pub user_id: Option<String>,
    
    pub resend_count: i32,
    pub expires_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub source: Source,
}

impl Registration {
    pub fn new(
        registration_id: String,
        email: String,
        username: String,
        verification_token: String,
        keycloak_id: String,
        expires_at: DateTime<Utc>,
        source: Source,
    ) -> Self {
        Self {
            registration_id,
            email,
            username,
            first_name: None,
            last_name: None,
            phone: None,
            verification_token,
            status: RegistrationStatus::Pending,
            keycloak_id,
            user_id: None,
            resend_count: 0,
            expires_at,
            verified_at: None,
            created_at: Utc::now(),
            ip_address: None,
            user_agent: None,
            source,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_verified(&self) -> bool {
        self.status == RegistrationStatus::Verified
    }

    pub fn verify(&mut self, user_id: String) {
        self.status = RegistrationStatus::Verified;
        self.verified_at = Some(Utc::now());
        self.user_id = Some(user_id);
    }

    pub fn increment_resend_count(&mut self) {
        self.resend_count += 1;
    }

    pub fn can_resend(&self) -> bool {
        self.resend_count < 5 && !self.is_expired()
    }
}

/// Refresh Token entity - OIDC refresh token lifecycle management
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub token_id: String,
    pub user_id: String,
    
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl RefreshToken {
    pub fn new(
        token_id: String,
        user_id: String,
        refresh_token: String,
        expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            token_id,
            user_id,
            refresh_token,
            expires_at,
            created_at: Utc::now(),
            revoked_at: None,
            ip_address: None,
            user_agent: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.revoked_at.is_none() && Utc::now() < self.expires_at
    }

    pub fn revoke(&mut self) {
        self.revoked_at = Some(Utc::now());
    }
}

/// User Invitation entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invitation {
    pub invitation_id: String,
    
    pub email: String,
    pub role: Role,
    
    pub network_id: Option<String>,
    pub station_id: Option<String>,
    
    pub invited_by: String,
    
    pub token: String,
    pub status: InvitationStatus,
    
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
}

impl Invitation {
    pub fn new(
        invitation_id: String,
        email: String,
        role: Role,
        invited_by: String,
        token: String,
        expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            invitation_id,
            email,
            role,
            network_id: None,
            station_id: None,
            invited_by,
            token,
            status: InvitationStatus::Pending,
            expires_at,
            accepted_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        self.status == InvitationStatus::Pending && !self.is_expired()
    }

    pub fn accept(&mut self) {
        self.status = InvitationStatus::Accepted;
        self.accepted_at = Some(Utc::now());
    }

    pub fn cancel(&mut self) {
        self.status = InvitationStatus::Cancelled;
    }
}

/// Login Audit Log entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoginAuditLog {
    pub log_id: i64,
    pub user_id: Option<String>,
    pub keycloak_id: Option<String>,
    
    pub action: String,
    pub action_details: Option<String>,
    
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    
    pub success: bool,
    pub error_message: Option<String>,
    
    pub created_at: DateTime<Utc>,
}

/// Keycloak Sync Log entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KeycloakSyncLog {
    pub log_id: i64,
    pub user_id: Option<String>,
    pub keycloak_id: Option<String>,
    
    pub action: String,
    pub status: String,
    
    pub details: Option<String>,
    pub error_message: Option<String>,
    
    pub created_at: DateTime<Utc>,
}

/// User Preferences entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreferences {
    pub user_id: String,
    
    pub language: String,
    pub timezone: String,
    
    pub notifications_enabled: bool,
    pub theme: String,
    
    pub preferences: Option<serde_json::Value>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            user_id: String::new(),
            language: "en".to_string(),
            timezone: "UTC".to_string(),
            notifications_enabled: true,
            theme: "light".to_string(),
            preferences: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Rate Limit entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RateLimit {
    pub id: i64,
    pub identifier: String,
    pub action: String,
    
    pub count: i32,
    
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    
    pub created_at: DateTime<Utc>,
}

impl RateLimit {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.window_end
    }
}