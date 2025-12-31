use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::entities::{
    User, Registration, RefreshToken, Invitation, LoginAuditLog,
    KeycloakSyncLog, UserPreferences, RateLimit,
};
use crate::domain::enums::{Role, RegistrationStatus, InvitationStatus, Source};
use crate::core::errors::AppError;

/// User Repository trait
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, AppError>;
    async fn get_by_id(&self, user_id: &str) -> Result<Option<User>, AppError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn get_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, AppError>;
    async fn get_all(&self) -> Result<Vec<User>, AppError>;
    async fn get_active_users(&self) -> Result<Vec<User>, AppError>;
    async fn update(&self, user: &User) -> Result<User, AppError>;
    async fn soft_delete(&self, user_id: &str) -> Result<(), AppError>;
    async fn update_last_login(&self, user_id: &str) -> Result<(), AppError>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, AppError>;
    async fn exists_by_username(&self, username: &str) -> Result<bool, AppError>;
    async fn count(&self) -> Result<i64, AppError>;
}

/// Registration Repository trait
#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    async fn create(&self, registration: &Registration) -> Result<Registration, AppError>;
    async fn get_by_id(&self, registration_id: &str) -> Result<Option<Registration>, AppError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<Registration>, AppError>;
    async fn get_by_token(&self, token: &str) -> Result<Option<Registration>, AppError>;
    async fn get_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<Registration>, AppError>;
    async fn update(&self, registration: &Registration) -> Result<Registration, AppError>;
    async fn update_status(&self, registration_id: &str, status: RegistrationStatus) -> Result<(), AppError>;
    async fn increment_resend_count(&self, registration_id: &str) -> Result<(), AppError>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, AppError>;
    async fn exists_by_username(&self, username: &str) -> Result<bool, AppError>;
    async fn delete_expired(&self) -> Result<u64, AppError>;
    async fn get_pending(&self) -> Result<Vec<Registration>, AppError>;
}

/// Refresh Token Repository trait
#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, token: &RefreshToken) -> Result<RefreshToken, AppError>;
    async fn get_by_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError>;
    async fn get_by_user_id(&self, user_id: &str) -> Result<Vec<RefreshToken>, AppError>;
    async fn revoke(&self, token_id: &str) -> Result<(), AppError>;
    async fn revoke_all_for_user(&self, user_id: &str) -> Result<(), AppError>;
    async fn is_valid(&self, token: &str) -> Result<bool, AppError>;
    async fn delete_expired(&self) -> Result<u64, AppError>;
}

/// Invitation Repository trait
#[async_trait]
pub trait InvitationRepository: Send + Sync {
    async fn create(&self, invitation: &Invitation) -> Result<Invitation, AppError>;
    async fn get_by_id(&self, invitation_id: &str) -> Result<Option<Invitation>, AppError>;
    async fn get_by_token(&self, token: &str) -> Result<Option<Invitation>, AppError>;
    async fn get_by_email(&self, email: &str) -> Result<Vec<Invitation>, AppError>;
    async fn get_all(&self) -> Result<Vec<Invitation>, AppError>;
    async fn get_pending(&self) -> Result<Vec<Invitation>, AppError>;
    async fn update(&self, invitation: &Invitation) -> Result<Invitation, AppError>;
    async fn update_status(&self, invitation_id: &str, status: InvitationStatus) -> Result<(), AppError>;
    async fn delete(&self, invitation_id: &str) -> Result<(), AppError>;
    async fn delete_expired(&self) -> Result<u64, AppError>;
    async fn count(&self) -> Result<i64, AppError>;
}

/// Login Audit Log Repository trait
#[async_trait]
pub trait LoginAuditLogRepository: Send + Sync {
    async fn create(&self, log: &LoginAuditLog) -> Result<LoginAuditLog, AppError>;
    async fn get_by_user_id(&self, user_id: &str, limit: i64) -> Result<Vec<LoginAuditLog>, AppError>;
    async fn get_failed_attempts(
        &self,
        identifier: &str,
        since: DateTime<Utc>,
    ) -> Result<Vec<LoginAuditLog>, AppError>;
    async fn delete_old_logs(&self, before: DateTime<Utc>) -> Result<u64, AppError>;
}

/// Keycloak Sync Log Repository trait
#[async_trait]
pub trait KeycloakSyncLogRepository: Send + Sync {
    async fn create(&self, log: &KeycloakSyncLog) -> Result<KeycloakSyncLog, AppError>;
    async fn get_by_user_id(&self, user_id: &str, limit: i64) -> Result<Vec<KeycloakSyncLog>, AppError>;
    async fn get_failed_syncs(&self, limit: i64) -> Result<Vec<KeycloakSyncLog>, AppError>;
    async fn delete_old_logs(&self, before: DateTime<Utc>) -> Result<u64, AppError>;
}

/// User Preferences Repository trait
#[async_trait]
pub trait UserPreferencesRepository: Send + Sync {
    async fn create(&self, preferences: &UserPreferences) -> Result<UserPreferences, AppError>;
    async fn get_by_user_id(&self, user_id: &str) -> Result<Option<UserPreferences>, AppError>;
    async fn update(&self, preferences: &UserPreferences) -> Result<UserPreferences, AppError>;
    async fn delete(&self, user_id: &str) -> Result<(), AppError>;
}

/// Rate Limit Repository trait
#[async_trait]
pub trait RateLimitRepository: Send + Sync {
    async fn get_or_create(
        &self,
        identifier: &str,
        action: &str,
        window_seconds: i64,
    ) -> Result<RateLimit, AppError>;
    
    async fn increment(
        &self,
        identifier: &str,
        action: &str,
    ) -> Result<i32, AppError>;
    
    async fn check_limit(
        &self,
        identifier: &str,
        action: &str,
        max_count: i32,
        window_seconds: i64,
    ) -> Result<bool, AppError>;
    
    async fn reset(
        &self,
        identifier: &str,
        action: &str,
    ) -> Result<(), AppError>;
    
    async fn delete_expired(&self) -> Result<u64, AppError>;
}

/// Email Verification Log Repository trait
#[async_trait]
pub trait EmailVerificationLogRepository: Send + Sync {
    async fn log_verification_attempt(
        &self,
        user_id: Option<&str>,
        email: &str,
        token: Option<&str>,
        verified: bool,
        ip_address: Option<&str>,
    ) -> Result<(), AppError>;
    
    async fn get_verification_history(
        &self,
        email: &str,
        limit: i64,
    ) -> Result<Vec<EmailVerificationLog>, AppError>;
}

/// Email Verification Log entity
#[derive(Debug, Clone)]
pub struct EmailVerificationLog {
    pub log_id: i64,
    pub user_id: Option<String>,
    pub email: String,
    pub verification_token: Option<String>,
    pub verified: bool,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}