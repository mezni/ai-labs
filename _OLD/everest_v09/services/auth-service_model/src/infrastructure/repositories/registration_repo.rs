use crate::core::errors::AppResult;
use crate::domain::entities::UserRegistration;
use crate::domain::repositories::RegistrationRepository;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgRegistrationRepository {
    pool: PgPool,
}

impl PgRegistrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegistrationRepository for PgRegistrationRepository {
    async fn create(&self, registration: &UserRegistration) -> AppResult<UserRegistration> {
        let result = sqlx::query_as::<_, UserRegistration>(
            r#"
            INSERT INTO user_registrations (
                registration_id, email, username, first_name, last_name, phone,
                verification_token, status, keycloak_id, expires_at,
                ip_address, user_agent, source
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
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
        .bind(&registration.status)
        .bind(&registration.keycloak_id)
        .bind(registration.expires_at)
        .bind(&registration.ip_address)
        .bind(&registration.user_agent)
        .bind(&registration.source)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, registration_id: &str) -> AppResult<Option<UserRegistration>> {
        let result = sqlx::query_as::<_, UserRegistration>(
            "SELECT * FROM user_registrations WHERE registration_id = $1",
        )
        .bind(registration_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<UserRegistration>> {
        let result = sqlx::query_as::<_, UserRegistration>(
            "SELECT * FROM user_registrations WHERE email = $1 ORDER BY created_at DESC LIMIT 1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_token(&self, token: &str) -> AppResult<Option<UserRegistration>> {
        let result = sqlx::query_as::<_, UserRegistration>(
            "SELECT * FROM user_registrations WHERE verification_token = $1",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, registration: &UserRegistration) -> AppResult<UserRegistration> {
        let result = sqlx::query_as::<_, UserRegistration>(
            r#"
            UPDATE user_registrations
            SET status = $2, user_id = $3, resend_count = $4,
                verified_at = $5, expires_at = $6
            WHERE registration_id = $1
            RETURNING *
            "#,
        )
        .bind(&registration.registration_id)
        .bind(&registration.status)
        .bind(&registration.user_id)
        .bind(registration.resend_count)
        .bind(registration.verified_at)
        .bind(registration.expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}
