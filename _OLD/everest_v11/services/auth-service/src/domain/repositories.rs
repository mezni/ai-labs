use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::core::errors::AppResult;
use crate::domain::entities::{Invitation, Registration, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        keycloak_id: &str,
        email: &str,
        username: &str,
        role: &str,
    ) -> AppResult<User>;
    async fn find_by_id(&self, id: &Uuid) -> AppResult<User>;
    async fn find_by_email(&self, email: &str) -> AppResult<User>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<User>;
    async fn find_all(&self) -> AppResult<Vec<User>>;
    async fn update(&self, user: &User) -> AppResult<User>;
    async fn soft_delete(&self, id: &Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    async fn create(
        &self,
        email: &str,
        username: &str,
        keycloak_id: &str,
        verification_expires_at: DateTime<Utc>,
    ) -> AppResult<Registration>;
    async fn find_by_id(&self, id: &Uuid) -> AppResult<Registration>;
    async fn find_by_email(&self, email: &str) -> AppResult<Registration>;
    async fn update_status(&self, id: &Uuid, status: &str) -> AppResult<()>;
    async fn increment_resend_count(&self, id: &Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait InvitationRepository: Send + Sync {
    async fn create(
        &self,
        code: &str,
        email: &str,
        role: &str,
        created_by: &Uuid,
        expires_at: DateTime<Utc>,
    ) -> AppResult<Invitation>;
    async fn find_by_id(&self, id: &Uuid) -> AppResult<Invitation>;
    async fn find_by_code(&self, code: &str) -> AppResult<Invitation>;
    async fn find_all(&self) -> AppResult<Vec<Invitation>>;
    async fn update(&self, invitation: &Invitation) -> AppResult<Invitation>;
}