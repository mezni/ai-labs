use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

// ============================================================================
// Email Value Object
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Email {
    value: String,
    normalized: String,
}

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        let trimmed = email.trim();
        if !Self::is_valid(trimmed) {
            return Err("Invalid email format".to_string());
        }
        Ok(Self {
            value: trimmed.to_string(),
            normalized: trimmed.to_lowercase(),
        })
    }

    fn is_valid(email: &str) -> bool {
        email.contains('@') && email.len() <= 255
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn normalized(&self) -> &str {
        &self.normalized
    }
}

// ============================================================================
// Username Value Object
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Username {
    value: String,
    normalized: String,
}

impl Username {
    pub fn new(username: String) -> Result<Self, String> {
        let trimmed = username.trim();
        if trimmed.len() < 3 || trimmed.len() > 100 {
            return Err("Username must be between 3 and 100 characters".to_string());
        }
        Ok(Self {
            value: trimmed.to_string(),
            normalized: trimmed.to_lowercase(),
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn normalized(&self) -> &str {
        &self.normalized
    }
}

// ============================================================================
// Phone Value Object
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Phone {
    value: String,
    normalized: String,
}

impl Phone {
    pub fn new(phone: String) -> Result<Self, String> {
        let normalized = Self::normalize(&phone);
        if normalized.len() > 20 {
            return Err("Phone number too long".to_string());
        }
        Ok(Self {
            value: phone,
            normalized,
        })
    }

    fn normalize(phone: &str) -> String {
        phone.chars().filter(|c| c.is_numeric()).collect()
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn normalized(&self) -> &str {
        &self.normalized
    }
}

// ============================================================================
// Registration Status
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegistrationStatus {
    Pending,
    Verified,
    Expired,
    Cancelled,
    Failed,
    KeycloakFailed,
    EmailFailed,
}

impl RegistrationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::KeycloakFailed => "keycloak_failed",
            Self::EmailFailed => "email_failed",
        }
    }
}

// ============================================================================
// User Status
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Pending,
    Active,
    Inactive,
    Locked,
    Suspended,
    Deleted,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Locked => "locked",
            Self::Suspended => "suspended",
            Self::Deleted => "deleted",
        }
    }
}

// ============================================================================
// Source
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Web,
    Mobile,
    Api,
    Admin,
    Invitation,
    Import,
    Sso,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::Mobile => "mobile",
            Self::Api => "api",
            Self::Admin => "admin",
            Self::Invitation => "invitation",
            Self::Import => "import",
            Self::Sso => "sso",
        }
    }
}

// ============================================================================
// Verification Method
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMethod {
    Email,
    Sms,
    Manual,
}

impl VerificationMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Sms => "sms",
            Self::Manual => "manual",
        }
    }
}

// ============================================================================
// Keycloak Sync Status
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KeycloakSyncStatus {
    Pending,
    InProgress,
    Success,
    Failed,
    Retry,
}

impl KeycloakSyncStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Retry => "retry",
        }
    }
}

// ============================================================================
// User Role
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Admin,
    Partner,
    Operator,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Admin => "admin",
            Self::Partner => "partner",
            Self::Operator => "operator",
        }
    }
}

// ============================================================================
// MFA Method
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MfaMethod {
    Totp,
    Sms,
    Email,
    Webauthn,
}

impl MfaMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Totp => "totp",
            Self::Sms => "sms",
            Self::Email => "email",
            Self::Webauthn => "webauthn",
        }
    }
}

// ============================================================================
// Gender
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::Female => "female",
            Self::Other => "other",
        }
    }
}

// ============================================================================
// Verification Context
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationContext {
    pub ip: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub verified_at: DateTime<Utc>,
}

// ============================================================================
// Registration Context
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationContext {
    pub source: Source,
    pub ip: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub campaign_id: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
}

// ============================================================================
// Keycloak Integration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakIntegration {
    pub keycloak_id: Option<String>,
    pub sync_status: KeycloakSyncStatus,
    pub sync_attempts: i32,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_error: Option<String>,
    pub sync_metadata: serde_json::Value,
}

impl Default for KeycloakIntegration {
    fn default() -> Self {
        Self {
            keycloak_id: None,
            sync_status: KeycloakSyncStatus::Pending,
            sync_attempts: 0,
            last_sync_at: None,
            sync_error: None,
            sync_metadata: serde_json::json!({}),
        }
    }
}

// ============================================================================
// Security Info
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityInfo {
    pub failed_login_attempts: i32,
    pub failed_password_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub last_failed_login: Option<DateTime<Utc>>,
}

impl Default for SecurityInfo {
    fn default() -> Self {
        Self {
            failed_login_attempts: 0,
            failed_password_attempts: 0,
            locked_until: None,
            last_failed_login: None,
        }
    }
}

// ============================================================================
// Consent Info
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentInfo {
    pub accepted_terms_at: Option<DateTime<Utc>>,
    pub accepted_privacy_at: Option<DateTime<Utc>>,
    pub marketing_consent: bool,
    pub data_processing_consent: bool,
}

impl Default for ConsentInfo {
    fn default() -> Self {
        Self {
            accepted_terms_at: None,
            accepted_privacy_at: None,
            marketing_consent: false,
            data_processing_consent: false,
        }
    }
}