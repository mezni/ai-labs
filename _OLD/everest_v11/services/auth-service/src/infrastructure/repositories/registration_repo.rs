use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::{constants::STATUS_PENDING_VERIFICATION, errors::AppResult};
use crate::domain::{entities::Registration, repositories::RegistrationRepository};

pub struct PostgresRegistrationRepository {
    pool: PgPool,
}

impl PostgresRegistrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegistrationRepository for PostgresRegistrationRepository {
    async fn create(
        &self,
        email: &str,
        username: &str,
        keycloak_id: &str,
        verification_expires_at: DateTime<Utc>,
    ) -> AppResult<Registration> {
        let registration = sqlx::query_as!(
            Registration,
            r#"
            INSERT INTO registrations (email, username, keycloak_id, status, verification_sent_at, verification_expires_at)
            VALUES ($1, $2, $3, $4, NOW(), $5)
            RETURNING id, email, username, keycloak_id, status, verification_sent_at, verification_expires_at,
                      resend_count, last_resend_at, created_at, updated_at
            "#,
            email,
            username,
            keycloak_id,
            STATUS_PENDING_VERIFICATION,
            verification_expires_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(registration)
    }

    async fn find_by_id(&self, id: &Uuid) -> AppResult<Registration> {
        let registration = sqlx::query_as!(
            Registration,
            r#"
            SELECT id, email, username, keycloak_id, status, verification_sent_at, verification_expires_at,
                   resend_count, last_resend_at, created_at, updated_at
            FROM registrations
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(registration)
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Registration> {
        let registration = sqlx::query_as!(
            Registration,
            r#"
            SELECT id, email, username, keycloak_id, status, verification_sent_at, verification_expires_at,
                   resend_count, last_resend_at, created_at, updated_at
            FROM registrations
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(registration)
    }

    async fn update_status(&self, id: &Uuid, status: &str) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE registrations
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            status,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn increment_resend_count(&self, id: &Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE registrations
            SET resend_count = resend_count + 1, last_resend_at = NOW(), updated_at = NOW()
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}