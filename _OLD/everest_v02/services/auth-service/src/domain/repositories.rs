use crate::{
    core::errors::AppError,
    domain::{user::User, user_registration::UserRegistration},
};
use async_trait::async_trait;

/// Repository for managing user registrations.
#[async_trait]
pub trait UserRegistrationRepository: Send + Sync {
    /// Saves a new registration or updates an existing one.
    async fn save(&self, registration: &UserRegistration) -> Result<(), AppError>;

    /// Finds a registration by its unique ID.
    async fn find_by_id(&self, id: &str) -> Result<Option<UserRegistration>, AppError>;

    /// Finds a registration by its verification token.
    async fn find_by_token(&self, token: &str) -> Result<Option<UserRegistration>, AppError>;

    /// Checks if an email or username is already taken.
    async fn exists_by_email_or_username(
        &self,
        email: &str,
        username: &str,
    ) -> Result<bool, AppError>;

    /// Deletes expired registrations (maintenance task).
    async fn delete_expired(&self) -> Result<u64, AppError>;
}

/// Repository for managing users.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Saves a user.
    async fn save(&self, user: &User) -> Result<(), AppError>;

    /// Finds a user by ID.
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError>;

    /// Finds a user by email.
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

    /// Finds a user by username.
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
}
