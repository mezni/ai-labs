use crate::domain::entities::{User, UserRegistration};
use crate::domain::value_objects::{Email, RegistrationStatus, UserStatus};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

// ============================================================================
// Repository Error Type
// ============================================================================

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    AlreadyExists,
    DatabaseError(String),
    ValidationError(String),
    ConcurrencyError,
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Resource not found"),
            Self::AlreadyExists => write!(f, "Resource already exists"),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::ConcurrencyError => write!(f, "Concurrency conflict"),
        }
    }
}

impl std::error::Error for RepositoryError {}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

// ============================================================================
// User Registration Repository
// ============================================================================

#[async_trait]
pub trait UserRegistrationRepository: Send + Sync {
    /// Create a new registration
    async fn create(&self, registration: &UserRegistration) -> RepositoryResult<()>;

    /// Find registration by ID
    async fn find_by_id(&self, registration_id: &str) -> RepositoryResult<UserRegistration>;

    /// Find registration by email (pending only)
    async fn find_by_email(&self, email: &Email) -> RepositoryResult<Option<UserRegistration>>;

    /// Find registration by verification token hash
    async fn find_by_token_hash(&self, token_hash: &str) -> RepositoryResult<Option<UserRegistration>>;

    /// Find registration by Keycloak ID
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> RepositoryResult<Option<UserRegistration>>;

    /// Update registration
    async fn update(&self, registration: &UserRegistration) -> RepositoryResult<()>;

    /// Delete registration
    async fn delete(&self, registration_id: &str) -> RepositoryResult<()>;

    /// Find registrations by status
    async fn find_by_status(&self, status: RegistrationStatus, limit: i32) -> RepositoryResult<Vec<UserRegistration>>;

    /// Find expired registrations
    async fn find_expired(&self, before: DateTime<Utc>, limit: i32) -> RepositoryResult<Vec<UserRegistration>>;

    /// Find registrations pending Keycloak sync
    async fn find_pending_keycloak_sync(&self, limit: i32) -> RepositoryResult<Vec<UserRegistration>>;

    /// Count registrations by status
    async fn count_by_status(&self, status: RegistrationStatus) -> RepositoryResult<i64>;
}

// ============================================================================
// User Repository
// ============================================================================

#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user
    async fn create(&self, user: &User) -> RepositoryResult<()>;

    /// Find user by ID
    async fn find_by_id(&self, user_id: &str) -> RepositoryResult<User>;

    /// Find user by Keycloak ID
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> RepositoryResult<Option<User>>;

    /// Find user by email
    async fn find_by_email(&self, email: &Email) -> RepositoryResult<Option<User>>;

    /// Find user by username
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<User>>;

    /// Find user by external ID
    async fn find_by_external_id(&self, external_id: &str) -> RepositoryResult<Option<User>>;

    /// Find user by registration ID
    async fn find_by_registration_id(&self, registration_id: &str) -> RepositoryResult<Option<User>>;

    /// Update user (with optimistic locking)
    async fn update(&self, user: &User) -> RepositoryResult<()>;

    /// Delete user (soft delete)
    async fn delete(&self, user_id: &str) -> RepositoryResult<()>;

    /// List users with pagination
    async fn list(&self, offset: i64, limit: i32) -> RepositoryResult<Vec<User>>;

    /// Find users by status
    async fn find_by_status(&self, status: UserStatus, offset: i64, limit: i32) -> RepositoryResult<Vec<User>>;

    /// Find users by role
    async fn find_by_role(&self, role: &str, offset: i64, limit: i32) -> RepositoryResult<Vec<User>>;

    /// Find locked users
    async fn find_locked(&self, limit: i32) -> RepositoryResult<Vec<User>>;

    /// Find inactive users (not logged in since date)
    async fn find_inactive_since(&self, since: DateTime<Utc>, limit: i32) -> RepositoryResult<Vec<User>>;

    /// Search users by query (email, username, name)
    async fn search(&self, query: &str, offset: i64, limit: i32) -> RepositoryResult<Vec<User>>;

    /// Count total users
    async fn count(&self) -> RepositoryResult<i64>;

    /// Count users by status
    async fn count_by_status(&self, status: UserStatus) -> RepositoryResult<i64>;

    /// Check if email exists
    async fn email_exists(&self, email: &Email) -> RepositoryResult<bool>;

    /// Check if username exists
    async fn username_exists(&self, username: &str) -> RepositoryResult<bool>;
}

// ============================================================================
// Repository Filters (for advanced queries)
// ============================================================================

#[derive(Debug, Clone)]
pub struct UserFilter {
    pub status: Option<UserStatus>,
    pub role: Option<String>,
    pub email_verified: Option<bool>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub last_login_after: Option<DateTime<Utc>>,
    pub last_login_before: Option<DateTime<Utc>>,
}

impl Default for UserFilter {
    fn default() -> Self {
        Self {
            status: None,
            role: None,
            email_verified: None,
            created_after: None,
            created_before: None,
            last_login_after: None,
            last_login_before: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegistrationFilter {
    pub status: Option<RegistrationStatus>,
    pub verified: Option<bool>,
    pub keycloak_synced: Option<bool>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub expires_after: Option<DateTime<Utc>>,
    pub expires_before: Option<DateTime<Utc>>,
}

impl Default for RegistrationFilter {
    fn default() -> Self {
        Self {
            status: None,
            verified: None,
            keycloak_synced: None,
            created_after: None,
            created_before: None,
            expires_after: None,
            expires_before: None,
        }
    }
}

// ============================================================================
// Extended Repository Trait (optional advanced features)
// ============================================================================

#[async_trait]
pub trait UserRepositoryExt: UserRepository {
    /// Find users with advanced filtering
    async fn find_with_filter(
        &self,
        filter: UserFilter,
        offset: i64,
        limit: i32,
    ) -> RepositoryResult<Vec<User>>;

    /// Bulk update user status
    async fn bulk_update_status(
        &self,
        user_ids: Vec<String>,
        status: UserStatus,
    ) -> RepositoryResult<i64>;
}

#[async_trait]
pub trait UserRegistrationRepositoryExt: UserRegistrationRepository {
    /// Find registrations with advanced filtering
    async fn find_with_filter(
        &self,
        filter: RegistrationFilter,
        offset: i64,
        limit: i32,
    ) -> RepositoryResult<Vec<UserRegistration>>;

    /// Bulk expire registrations
    async fn bulk_expire(&self, registration_ids: Vec<String>) -> RepositoryResult<i64>;
}