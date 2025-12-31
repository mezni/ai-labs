use super::entities::{User, UserRegistration};
use crate::core::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<User>;
    async fn find_by_id(&self, user_id: &str) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<Option<User>>;
    async fn update(&self, user: &User) -> AppResult<User>;
    async fn update_last_login(&self, user_id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    async fn create(&self, registration: &UserRegistration) -> AppResult<UserRegistration>;
    async fn find_by_id(&self, registration_id: &str) -> AppResult<Option<UserRegistration>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<UserRegistration>>;
    async fn find_by_token(&self, token: &str) -> AppResult<Option<UserRegistration>>;
    async fn update(&self, registration: &UserRegistration) -> AppResult<UserRegistration>;
}
