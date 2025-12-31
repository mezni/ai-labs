use crate::core::errors::AppError;
use crate::domain::entities::{RefreshToken, User, UserRegistration};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, AppError>;
    async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, AppError>;
    async fn update_last_login(&self, user_id: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    async fn create(&self, registration: &UserRegistration) -> Result<UserRegistration, AppError>;
    async fn find_by_token(&self, token: &str) -> Result<Option<UserRegistration>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserRegistration>, AppError>;
    async fn update_status(&self, registration_id: &str, status: &str) -> Result<(), AppError>;
    async fn update_user_id(&self, registration_id: &str, user_id: &str) -> Result<(), AppError>;
    async fn increment_resend_count(&self, registration_id: &str) -> Result<(), AppError>;
    async fn update_verification_token(
        &self,
        registration_id: &str,
        token: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError>;
}

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, token: &RefreshToken) -> Result<RefreshToken, AppError>;
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError>;
    async fn revoke(&self, token_id: &str) -> Result<(), AppError>;
    async fn delete_expired(&self) -> Result<u64, AppError>;
}
