use crate::domain::entities::{User, UserRole};
use crate::infrastructure::error::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        user_id: &str,
        keycloak_id: &str,
        email: &str,
        username: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        phone: Option<&str>,
        photo: Option<&str>,
        role: &str,
        network_id: &str,
        station_id: &str,
        source: &str,
        created_by: Option<&str>,
    ) -> Result<User, DomainError>;

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError>;
    async fn list_users(
        &self,
        role: Option<UserRole>,
        is_active: Option<bool>,
    ) -> Result<Vec<User>, DomainError>;

    async fn update_user(
        &self,
        user_id: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        phone: Option<&str>,
        photo: Option<&str>,
        updated_by: Option<&str>,
    ) -> Result<User, DomainError>;

    async fn update_password_changed(&self, user_id: &str) -> Result<(), DomainError>;
    async fn deactivate_user(
        &self,
        user_id: &str,
        updated_by: Option<&str>,
    ) -> Result<(), DomainError>;
    async fn verify_user(&self, user_id: &str) -> Result<(), DomainError>;
}
