use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::{constants::STATUS_ACTIVE, errors::AppResult};
use crate::domain::{entities::User, repositories::UserRepository};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(
        &self,
        keycloak_id: &str,
        email: &str,
        username: &str,
        role: &str,
    ) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (keycloak_id, email, username, status, role)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, keycloak_id, email, username, status, role, created_at, updated_at, deleted_at
            "#,
            keycloak_id,
            email,
            username,
            STATUS_ACTIVE,
            role
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &Uuid) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, keycloak_id, email, username, status, role, created_at, updated_at, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, keycloak_id, email, username, status, role, created_at, updated_at, deleted_at
            FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, keycloak_id, email, username, status, role, created_at, updated_at, deleted_at
            FROM users
            WHERE keycloak_id = $1 AND deleted_at IS NULL
            "#,
            keycloak_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_all(&self) -> AppResult<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, keycloak_id, email, username, status, role, created_at, updated_at, deleted_at
            FROM users
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn update(&self, user: &User) -> AppResult<User> {
        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $1, username = $2, status = $3, role = $4, updated_at = $5
            WHERE id = $6
            RETURNING id, keycloak_id, email, username, status, role, created_at, updated_at, deleted_at
            "#,
            user.email,
            user.username,
            user.status,
            user.role,
            user.updated_at,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    async fn soft_delete(&self, id: &Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET deleted_at = NOW()
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}