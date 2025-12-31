use crate::domain::value_objects::*;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

// ============================================================================
// User Registration Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistration {
    // Primary Identifier
    pub registration_id: String,

    // User Details
    pub email: Email,
    pub username: Option<Username>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<Phone>,

    // Verification
    pub verification_token_hash: String,
    pub verification_code: Option<String>,
    pub verification_token_expires_at: DateTime<Utc>,
    pub verification_method: VerificationMethod,
    pub verified_at: Option<DateTime<Utc>>,
    pub verification_context: Option<VerificationContext>,

    // Keycloak Integration
    pub keycloak: KeycloakIntegration,

    // Source & Context
    pub context: RegistrationContext,

    // Status
    pub status: RegistrationStatus,
    pub status_reason: Option<String>,

    // Lifecycle Expiry
    pub expires_at: DateTime<Utc>,

    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserRegistration {
    pub fn new(
        email: Email,
        verification_token_hash: String,
        verification_method: VerificationMethod,
        context: RegistrationContext,
        expires_at: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            registration_id: uuid::Uuid::new_v4().to_string().replace("-", ""),
            email,
            username: None,
            first_name: None,
            last_name: None,
            phone: None,
            verification_token_hash,
            verification_code: None,
            verification_token_expires_at: now + chrono::Duration::hours(24),
            verification_method,
            verified_at: None,
            verification_context: None,
            keycloak: KeycloakIntegration::default(),
            context,
            status: RegistrationStatus::Pending,
            status_reason: None,
            expires_at,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_verification_token_expired(&self) -> bool {
        Utc::now() > self.verification_token_expires_at
    }

    pub fn is_verified(&self) -> bool {
        self.verified_at.is_some() && self.status == RegistrationStatus::Verified
    }

    pub fn can_verify(&self) -> bool {
        self.status == RegistrationStatus::Pending
            && !self.is_verification_token_expired()
            && !self.is_expired()
    }

    pub fn verify(&mut self, context: VerificationContext) -> Result<(), String> {
        if !self.can_verify() {
            return Err("Registration cannot be verified".to_string());
        }

        self.status = RegistrationStatus::Verified;
        self.verified_at = Some(context.verified_at);
        self.verification_context = Some(context);
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn cancel(&mut self, reason: Option<String>) {
        self.status = RegistrationStatus::Cancelled;
        self.status_reason = reason;
        self.updated_at = Utc::now();
    }

    pub fn mark_expired(&mut self) {
        self.status = RegistrationStatus::Expired;
        self.updated_at = Utc::now();
    }

    pub fn mark_keycloak_synced(&mut self, keycloak_id: String) {
        self.keycloak.keycloak_id = Some(keycloak_id);
        self.keycloak.sync_status = KeycloakSyncStatus::Success;
        self.keycloak.last_sync_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn mark_keycloak_failed(&mut self, error: String) {
        self.keycloak.sync_status = KeycloakSyncStatus::Failed;
        self.keycloak.sync_attempts += 1;
        self.keycloak.sync_error = Some(error);
        self.keycloak.last_sync_at = Some(Utc::now());
        self.status = RegistrationStatus::KeycloakFailed;
        self.updated_at = Utc::now();
    }
}

// ============================================================================
// User Entity
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    // Primary Identifiers
    pub user_id: String,
    pub keycloak_id: String,
    pub external_id: Option<String>,

    // Traceability
    pub registration_id: Option<String>,

    // Personal Information
    pub email: Email,
    pub username: Option<Username>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<Phone>,
    pub avatar_url: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<Gender>,

    // Authorization
    pub role: UserRole,

    // Status Flags
    pub status: UserStatus,
    pub status_reason: Option<String>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub requires_password_change: bool,

    // MFA
    pub mfa_method: Option<MfaMethod>,

    // Security & Lockout
    pub security: SecurityInfo,

    // Source & Metadata
    pub source: Source,
    pub source_details: serde_json::Value,
    pub registration_ip: Option<IpAddr>,
    pub registration_user_agent: Option<String>,

    // Activity Tracking
    pub last_login_at: Option<DateTime<Utc>>,
    pub last_login_ip: Option<IpAddr>,
    pub last_login_user_agent: Option<String>,
    pub last_activity_at: DateTime<Utc>,
    pub login_count: i32,

    // Privacy & Compliance
    pub consent: ConsentInfo,
    pub gdpr_anonymized_at: Option<DateTime<Utc>>,

    // Audit & Lifecycle
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
    pub version: i32,
}

impl User {
    pub fn new(
        keycloak_id: String,
        email: Email,
        registration_id: Option<String>,
        source: Source,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id: uuid::Uuid::new_v4().to_string().replace("-", ""),
            keycloak_id,
            external_id: None,
            registration_id,
            email,
            username: None,
            first_name: None,
            last_name: None,
            phone: None,
            avatar_url: None,
            date_of_birth: None,
            gender: None,
            role: UserRole::User,
            status: UserStatus::Pending,
            status_reason: None,
            email_verified: false,
            phone_verified: false,
            requires_password_change: false,
            mfa_method: None,
            security: SecurityInfo::default(),
            source,
            source_details: serde_json::json!({}),
            registration_ip: None,
            registration_user_agent: None,
            last_login_at: None,
            last_login_ip: None,
            last_login_user_agent: None,
            last_activity_at: now,
            login_count: 0,
            consent: ConsentInfo::default(),
            gdpr_anonymized_at: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            archived_at: None,
            version: 1,
        }
    }

    pub fn display_name(&self) -> String {
        if let (Some(first), Some(last)) = (&self.first_name, &self.last_name) {
            format!("{} {}", first, last)
        } else if let Some(username) = &self.username {
            username.value().to_string()
        } else {
            self.email.value().to_string()
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active && self.deleted_at.is_none()
    }

    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.security.locked_until {
            Utc::now() < locked_until
        } else {
            false
        }
    }

    pub fn activate(&mut self) {
        self.status = UserStatus::Active;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn suspend(&mut self, reason: Option<String>) {
        self.status = UserStatus::Suspended;
        self.status_reason = reason;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn lock(&mut self, duration: chrono::Duration) {
        self.status = UserStatus::Locked;
        self.security.locked_until = Some(Utc::now() + duration);
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn unlock(&mut self) {
        if self.status == UserStatus::Locked {
            self.status = UserStatus::Active;
            self.security.locked_until = None;
            self.updated_at = Utc::now();
            self.version += 1;
        }
    }

    pub fn record_login(&mut self, ip: Option<IpAddr>, user_agent: Option<String>) {
        self.last_login_at = Some(Utc::now());
        self.last_login_ip = ip;
        self.last_login_user_agent = user_agent;
        self.last_activity_at = Utc::now();
        self.login_count += 1;
        self.security.failed_login_attempts = 0;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn record_failed_login(&mut self) {
        self.security.failed_login_attempts += 1;
        self.security.last_failed_login = Some(Utc::now());
        self.updated_at = Utc::now();
        self.version += 1;

        // Auto-lock after 5 failed attempts
        if self.security.failed_login_attempts >= 5 {
            self.lock(chrono::Duration::hours(1));
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity_at = Utc::now();
        self.updated_at = Utc::now();
    }

    pub fn soft_delete(&mut self) {
        self.status = UserStatus::Deleted;
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn verify_phone(&mut self) {
        self.phone_verified = true;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn enable_mfa(&mut self, method: MfaMethod) {
        self.mfa_method = Some(method);
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn disable_mfa(&mut self) {
        self.mfa_method = None;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn accept_terms(&mut self) {
        self.consent.accepted_terms_at = Some(Utc::now());
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn accept_privacy(&mut self) {
        self.consent.accepted_privacy_at = Some(Utc::now());
        self.updated_at = Utc::now();
        self.version += 1;
    }
}