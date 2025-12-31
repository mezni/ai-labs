use crate::core::errors::AppError;
use crate::domain::repositories::UserRegistrationRepository;
use crate::domain::user_registration::{RegistrationStatus, UserRegistration};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub struct PostgresUserRegistrationRepository {
    pool: PgPool,
}

impl PostgresUserRegistrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRegistrationRepository for PostgresUserRegistrationRepository {
    async fn save(&self, reg: &UserRegistration) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO user_registrations 
            (
                registration_id, email, username, first_name, last_name, phone, 
                verification_token, verification_code, status, keycloak_id, user_id, 
                expires_at, verified_at, created_at, updated_at, ip_address, user_agent
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (registration_id) DO UPDATE SET
                status = EXCLUDED.status,
                keycloak_id = EXCLUDED.keycloak_id,
                user_id = EXCLUDED.user_id,
                verified_at = EXCLUDED.verified_at,
                updated_at = EXCLUDED.updated_at,
                verification_code = EXCLUDED.verification_code,
                ip_address = EXCLUDED.ip_address,
                user_agent = EXCLUDED.user_agent
            "#,
            reg.registration_id,
            reg.email,
            reg.username,
            reg.first_name,
            reg.last_name,
            reg.phone,
            reg.verification_token,
            reg.verification_code,
            reg.status.clone() as RegistrationStatus,
            reg.keycloak_id,
            reg.user_id,
            reg.expires_at.naive_utc(),
            reg.verified_at.map(|dt| dt.naive_utc()),
            reg.created_at.naive_utc(),
            reg.updated_at.naive_utc(),
            reg.ip_address,
            reg.user_agent
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<UserRegistration>, AppError> {
        let result = sqlx::query_as!(
            UserRegistration,
            r#"
            SELECT 
                registration_id as "registration_id!", 
                email as "email!", 
                username as "username!", 
                first_name, 
                last_name, 
                phone, 
                verification_token as "verification_token!", 
                verification_code,
                status as "status: RegistrationStatus", 
                keycloak_id, 
                user_id, 
                expires_at as "expires_at!: DateTime<Utc>",
                verified_at as "verified_at: DateTime<Utc>", 
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                ip_address,
                user_agent
            FROM user_registrations 
            WHERE registration_id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<UserRegistration>, AppError> {
        let result = sqlx::query_as!(
            UserRegistration,
            r#"
            SELECT 
                registration_id as "registration_id!", 
                email as "email!", 
                username as "username!", 
                first_name, 
                last_name, 
                phone, 
                verification_token as "verification_token!", 
                verification_code,
                status as "status: RegistrationStatus", 
                keycloak_id, 
                user_id, 
                expires_at as "expires_at!: DateTime<Utc>",
                verified_at as "verified_at: DateTime<Utc>", 
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                ip_address,
                user_agent
            FROM user_registrations 
            WHERE verification_token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn exists_by_email_or_username(
        &self,
        email: &str,
        username: &str,
    ) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"SELECT EXISTS(SELECT 1 FROM user_registrations WHERE email = $1 OR username = $2) as "exists!""#,
            email,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.exists)
    }

    async fn delete_expired(&self) -> Result<u64, AppError> {
        let result = sqlx::query!(
            "DELETE FROM user_registrations WHERE expires_at < NOW() AND status = 'pending'"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected())
    }
}
