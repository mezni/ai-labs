// src/domain/repositories.rs
use crate::core::errors::AppResult;
use crate::domain::{registration::UserRegistration, user::User};
use async_trait::async_trait;

/// User repository trait - defines the contract for user persistence
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Find user by ID
    async fn find_by_id(&self, user_id: &str) -> AppResult<Option<User>>;

    /// Find user by email
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;

    /// Find user by username
    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>>;

    /// Find user by Keycloak ID
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<Option<User>>;

    /// Save a new user
    async fn save(&self, user: &User) -> AppResult<()>;

    /// Update an existing user
    async fn update(&self, user: &User) -> AppResult<()>;

    /// List users with optional filters
    async fn list(&self, filters: UserFilters) -> AppResult<Vec<User>>;

    /// Count users with optional filters
    async fn count(&self, filters: UserFilters) -> AppResult<i64>;

    /// Check if email exists
    async fn email_exists(&self, email: &str) -> AppResult<bool>;

    /// Check if username exists
    async fn username_exists(&self, username: &str) -> AppResult<bool>;

    /// Get users by role
    async fn find_by_role(&self, role: &str) -> AppResult<Vec<User>>;

    /// Get users by network ID
    async fn find_by_network_id(&self, network_id: &str) -> AppResult<Vec<User>>;

    /// Get users by station ID
    async fn find_by_station_id(&self, station_id: &str) -> AppResult<Vec<User>>;
}

/// Registration repository trait
#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    /// Find registration by ID
    async fn find_by_id(&self, registration_id: &str) -> AppResult<Option<UserRegistration>>;

    /// Find registration by email
    async fn find_by_email(&self, email: &str) -> AppResult<Option<UserRegistration>>;

    /// Find registration by verification token
    async fn find_by_token(&self, token: &str) -> AppResult<Option<UserRegistration>>;

    /// Save a new registration
    async fn save(&self, registration: &UserRegistration) -> AppResult<()>;

    /// Update registration
    async fn update(&self, registration: &UserRegistration) -> AppResult<()>;

    /// Delete old registrations (cleanup)
    async fn delete_expired(&self) -> AppResult<u64>;

    /// Find pending registrations by email
    async fn find_pending_by_email(&self, email: &str) -> AppResult<Option<UserRegistration>>;
}

/// Audit log repository trait
#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    /// Log a login event
    async fn log_login(
        &self,
        user_id: &str,
        keycloak_id: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
        success: bool,
    ) -> AppResult<()>;

    /// Log a logout event
    async fn log_logout(&self, user_id: &str) -> AppResult<()>;

    /// Log a password change
    async fn log_password_change(&self, user_id: &str, changed_by: &str) -> AppResult<()>;

    /// Log a password reset request
    async fn log_password_reset_request(
        &self,
        user_id: &str,
        email: &str,
        ip_address: Option<String>,
    ) -> AppResult<()>;

    /// Log a password reset completion
    async fn log_password_reset(&self, user_id: &str) -> AppResult<()>;

    /// Log an email verification
    async fn log_email_verification(&self, user_id: &str, email: &str) -> AppResult<()>;

    /// Log a user creation
    async fn log_user_creation(
        &self,
        user_id: &str,
        created_by: Option<&str>,
        action_details: &str,
    ) -> AppResult<()>;

    /// Log a user update
    async fn log_user_update(
        &self,
        user_id: &str,
        updated_by: &str,
        action_details: &str,
    ) -> AppResult<()>;

    /// Log a user deletion
    async fn log_user_deletion(&self, user_id: &str, deleted_by: &str) -> AppResult<()>;

    /// Log a role change
    async fn log_role_change(
        &self,
        user_id: &str,
        old_role: &str,
        new_role: &str,
        changed_by: &str,
    ) -> AppResult<()>;

    /// Get login audit logs with pagination
    async fn get_login_logs(&self, filters: AuditLogFilters) -> AppResult<Vec<LoginAuditLog>>;

    /// Count login audit logs
    async fn count_login_logs(&self, filters: AuditLogFilters) -> AppResult<i64>;
}

// Filter structs
#[derive(Debug, Clone, Default)]
pub struct UserFilters {
    pub search: Option<String>,
    pub role: Option<String>,
    pub source: Option<String>,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub include_deleted: bool,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Desc
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuditLogFilters {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub from_date: Option<chrono::DateTime<chrono::Utc>>,
    pub to_date: Option<chrono::DateTime<chrono::Utc>>,
    pub success: Option<bool>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

// Audit log models
#[derive(Debug, Clone)]
pub struct LoginAuditLog {
    pub log_id: i64,
    pub user_id: String,
    pub keycloak_id: String,
    pub action: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
