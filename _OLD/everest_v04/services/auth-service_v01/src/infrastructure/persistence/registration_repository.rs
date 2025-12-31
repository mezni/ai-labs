use crate::core::{database::DbPool, errors::AppError};
use crate::domain::{
    entities::UserRegistration,
    repositories::RegistrationRepository,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

pub struct RegistrationRepositoryImpl {
    pool: DbPool,
}

impl RegistrationRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegistrationRepository for RegistrationRepositoryImpl {
    async fn create(&self, registration: &UserRegistration) -> Result<UserRegistration, AppError> {
        let row = sqlx::query_as::<_, UserRegistration>(
                r#"
                INSERT INTO user_registrations (
                    registration_id, email, username, first_name, last_name, phone,
                    verification_token, status, keycloak_id, expires_at, created_at,
                    ip_address, user_agent, source
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                RETURNING *
                "#,
            )
            .bind(&registration.registration_id)
            .bind(&registration.email)
            .bind(&registration.username)
            .bind(&registration.first_name)
            .bind(&registration.last_name)
            .bind(&registration.phone)
            .bind(&registration.verification_token)
            .bind(&registration.status.to_string())
            .bind(&registration.keycloak_id)
            .bind(&registration.expires_at)
            .bind(&registration.created_at)
            .bind(&registration.ip_address)
            .bind(&registration.user_agent)
            .bind(&registration.source.to_string())
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<UserRegistration>, AppError> {
        let result = sqlx::query_as::<_, UserRegistration>(
                "SELECT * FROM user_registrations WHERE verification_token = $1",
            )
            .bind(token)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserRegistration>, AppError> {
        let result = sqlx::query_as::<_, UserRegistration>(
                "SELECT * FROM user_registrations WHERE email = $1 AND status = 'pending' ORDER BY created_at DESC LIMIT 1",
            )
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn update_status(&self, registration_id: &str, status: &str) -> Result<(), AppError> {
        sqlx::query(
                "UPDATE user_registrations SET status = $1, verified_at = $2 WHERE registration_id = $3",
            )
            .bind(status)
            .bind(Utc::now())
            .bind(registration_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_user_id(&self, registration_id: &str, user_id: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE user_registrations SET user_id = $1 WHERE registration_id = $2")
            .bind(user_id)
            .bind(registration_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn increment_resend_count(&self, registration_id: &str) -> Result<(), AppError> {
        sqlx::query(
                "UPDATE user_registrations SET resend_count = resend_count + 1 WHERE registration_id = $1",
            )
            .bind(registration_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_verification_token(
        &self,
        registration_id: &str,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), AppError> {
        sqlx::query(
                "UPDATE user_registrations SET verification_token = $1, expires_at = $2 WHERE registration_id = $3",
            )
            .bind(token)
            .bind(expires_at)
            .bind(registration_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}