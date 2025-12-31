use crate::domain::entities::{AuditLog, User};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<User, sqlx::Error>;
    async fn find_user_by_id(&self, id: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_user_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, sqlx::Error>;
    async fn list_users(&self, limit: i64, offset: i64, search: Option<String>) -> Result<Vec<User>, sqlx::Error>;
    async fn update_user(&self, user: &User) -> Result<User, sqlx::Error>;
    async fn soft_delete_user(&self, id: &str) -> Result<(), sqlx::Error>;
}

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn create_audit_log(&self, log: &AuditLog) -> Result<AuditLog, sqlx::Error>;
}