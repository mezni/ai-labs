// src/infrastructure/registration_repo.rs
use crate::core::errors::AppResult;
use crate::domain::{
    registration::{RegistrationStatus, UserRegistration},
    repositories::RegistrationRepository,
    value_objects::*,
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::str::FromStr;

pub struct PostgresRegistrationRepository {
    pool: PgPool,
}

impl PostgresRegistrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn row_to_registration(row: &sqlx::postgres::PgRow) -> AppResult<UserRegistration> {
        use sqlx::Row;

        let email = Email::new(row.get("email"))?;
        let username = Username::new(row.get("username"))?;

        let first_name: Option<String> = row.get("first_name");
        let last_name: Option<String> = row.get("last_name");
        let name = PersonName::new(first_name, last_name)?;

        let phone: Option<String> = row.get("phone");
        let phone = PhoneNumber::new(phone)?;

        let status_str: String = row.get("status");
        let status = RegistrationStatus::from_str(&status_str)?;

        Ok(UserRegistration {
            registration_id: row.get("registration_id"),
            email,
            username,
            name,
            phone,
            verification_token: row.get("verification_token"),
            verification_code: row.get("verification_code"),
            status,
            keycloak_id: row.get("keycloak_id"),
            user_id: row.get("user_id"),
            expires_at: row.get("expires_at"),
            verified_at: row.get("verified_at"),
            created_at: row.get("created_at"),
            ip_address: row.get("ip_address"),
            user_agent: row.get("user_agent"),
        })
    }
}

#[async_trait]
impl RegistrationRepository for PostgresRegistrationRepository {
    async fn find_by_id(&self, registration_id: &str) -> AppResult<Option<UserRegistration>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM user_registrations
            WHERE registration_id = $1
            "#,
        )
        .bind(registration_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_registration(&r)?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<UserRegistration>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM user_registrations
            WHERE LOWER(email) = LOWER($1)
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_registration(&r)?)),
            None => Ok(None),
        }
    }

    async fn find_by_token(&self, token: &str) -> AppResult<Option<UserRegistration>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM user_registrations
            WHERE verification_token = $1
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_registration(&r)?)),
            None => Ok(None),
        }
    }

    async fn save(&self, registration: &UserRegistration) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO user_registrations (
                registration_id, email, username, first_name, last_name,
                phone, verification_token, verification_code, status,
                expires_at, created_at, ip_address, user_agent
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(&registration.registration_id)
        .bind(registration.email.as_str())
        .bind(registration.username.as_str())
        .bind(registration.name.first_name())
        .bind(registration.name.last_name())
        .bind(registration.phone.as_str())
        .bind(&registration.verification_token)
        .bind(&registration.verification_code)
        .bind(registration.status.as_str())
        .bind(registration.expires_at)
        .bind(registration.created_at)
        .bind(&registration.ip_address)
        .bind(&registration.user_agent)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, registration: &UserRegistration) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE user_registrations
            SET status = $2, keycloak_id = $3, user_id = $4, verified_at = $5
            WHERE registration_id = $1
            "#,
        )
        .bind(&registration.registration_id)
        .bind(registration.status.as_str())
        .bind(&registration.keycloak_id)
        .bind(&registration.user_id)
        .bind(registration.verified_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_expired(&self) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM user_registrations
            WHERE expires_at < NOW() AND status = 'pending'
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn find_pending_by_email(&self, email: &str) -> AppResult<Option<UserRegistration>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM user_registrations
            WHERE LOWER(email) = LOWER($1) AND status = 'pending'
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_registration(&r)?)),
            None => Ok(None),
        }
    }
}
