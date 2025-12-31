use async_trait::async_trait;
use crate::core::errors::AppError;
use crate::domain::user_registration::UserRegistration;

#[async_trait]
pub trait UserRegistrationRepository: Send + Sync {
    /// Save a new registration or update an existing one
    async fn save(&self, registration: &UserRegistration) -> Result<(), AppError>;

    /// Find a registration by its unique ID
    async fn find_by_id(&self, id: &str) -> Result<Option<UserRegistration>, AppError>;

    /// Find by the verification token sent via email
    async fn find_by_token(&self, token: &str) -> Result<Option<UserRegistration>, AppError>;

    /// Check if an email or username is already taken
    async fn exists_by_email_or_username(&self, email: &str, username: &str) -> Result<bool, AppError>;

    /// Delete expired registrations (Maintenance)
    async fn delete_expired(&self) -> Result<u64, AppError>;
}