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
    async fn create(&self, reg: &UserRegistration) -> AppResult<UserRegistration> {
        // Included user_id in insert since it's in the struct,
        // though it's likely NULL at creation time.
        let result = sqlx::query_as::<_, UserRegistration>(
            r#"
            INSERT INTO user_registrations (
                registration_id, email, username, first_name, last_name, phone,
                verification_token, status, keycloak_id, user_id, expires_at,
                ip_address, user_agent, source
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#,
        )
        .bind(&reg.registration_id)
        .bind(&reg.email)
        .bind(&reg.username)
        .bind(&reg.first_name)
        .bind(&reg.last_name)
        .bind(&reg.phone)
        .bind(&reg.verification_token)
        .bind(&reg.status)
        .bind(&reg.keycloak_id) // Option<String> maps to NULL automatically
        .bind(&reg.user_id) // Option<String> maps to NULL automatically
        .bind(reg.expires_at)
        .bind(&reg.ip_address)
        .bind(&reg.user_agent)
        .bind(&reg.source)
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
        // Added ORDER BY to ensure we get the most recent registration attempt
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

    async fn update(&self, reg: &UserRegistration) -> AppResult<UserRegistration> {
        let result = sqlx::query_as::<_, UserRegistration>(
            r#"
            UPDATE user_registrations
            SET status = $2, 
                keycloak_id = $3,
                user_id = $4, 
                resend_count = $5,
                verified_at = $6, 
                expires_at = $7
            WHERE registration_id = $1
            RETURNING *
            "#,
        )
        .bind(&reg.registration_id)
        .bind(&reg.status)
        .bind(&reg.keycloak_id)
        .bind(&reg.user_id)
        .bind(reg.resend_count)
        .bind(reg.verified_at)
        .bind(reg.expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}
