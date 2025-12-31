use crate::core::errors::AppError;
use crate::domain::user::{UserRole, UserSource};
use crate::domain::{RegistrationStatus, User, UserRegistration};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    async fn create(&self, registration: &UserRegistration) -> Result<(), AppError>;
    async fn find_by_id(&self, registration_id: &str)
        -> Result<Option<UserRegistration>, AppError>;
    async fn find_by_keycloak_id(
        &self,
        keycloak_id: &str,
    ) -> Result<Option<UserRegistration>, AppError>;
    async fn update_keycloak_id(
        &self,
        registration_id: &str,
        keycloak_id: &str,
    ) -> Result<(), AppError>;
    async fn mark_verified(&self, registration_id: &str, user_id: &str) -> Result<(), AppError>;
    async fn find_expired(&self) -> Result<Vec<UserRegistration>, AppError>;
    async fn mark_expired(&self, registration_ids: Vec<String>) -> Result<(), AppError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
}

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
    async fn create(&self, registration: &UserRegistration) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO user_registrations 
            (registration_id, email, username, first_name, last_name, phone, 
             verification_token, status, expires_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            registration.registration_id,
            registration.email,
            registration.username,
            registration.first_name,
            registration.last_name,
            registration.phone,
            registration.verification_token,
            registration.status.to_string(),
            registration.expires_at.naive_utc(),
            registration.created_at.naive_utc(),
            registration.updated_at.naive_utc(),
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        registration_id: &str,
    ) -> Result<Option<UserRegistration>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT registration_id, email, username, first_name, last_name, phone,
                   verification_token, status, keycloak_id, user_id, expires_at,
                   verified_at, created_at, updated_at
            FROM user_registrations
            WHERE registration_id = $1
            "#,
        )
        .bind(registration_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            use chrono::TimeZone;
            UserRegistration {
                registration_id: r.get("registration_id"),
                email: r.get("email"),
                username: r.get("username"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                phone: r.get("phone"),
                verification_token: r.get("verification_token"),
                status: parse_status(r.get("status")),
                keycloak_id: r.get("keycloak_id"),
                user_id: r.get("user_id"),
                expires_at: Utc.from_utc_datetime(&r.get("expires_at")),
                verified_at: r
                    .get::<Option<chrono::NaiveDateTime>, _>("verified_at")
                    .map(|dt| Utc.from_utc_datetime(&dt)),
                created_at: Utc.from_utc_datetime(&r.get("created_at")),
                updated_at: Utc.from_utc_datetime(&r.get("updated_at")),
            }
        }))
    }

    async fn find_by_keycloak_id(
        &self,
        keycloak_id: &str,
    ) -> Result<Option<UserRegistration>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT registration_id, email, username, first_name, last_name, phone,
                   verification_token, status, keycloak_id, user_id, expires_at,
                   verified_at, created_at, updated_at
            FROM user_registrations
            WHERE keycloak_id = $1
            "#,
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            use chrono::TimeZone;
            UserRegistration {
                registration_id: r.get("registration_id"),
                email: r.get("email"),
                username: r.get("username"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                phone: r.get("phone"),
                verification_token: r.get("verification_token"),
                status: parse_status(r.get("status")),
                keycloak_id: r.get("keycloak_id"),
                user_id: r.get("user_id"),
                expires_at: Utc.from_utc_datetime(&r.get("expires_at")),
                verified_at: r
                    .get::<Option<chrono::NaiveDateTime>, _>("verified_at")
                    .map(|dt| Utc.from_utc_datetime(&dt)),
                created_at: Utc.from_utc_datetime(&r.get("created_at")),
                updated_at: Utc.from_utc_datetime(&r.get("updated_at")),
            }
        }))
    }

    async fn update_keycloak_id(
        &self,
        registration_id: &str,
        keycloak_id: &str,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE user_registrations
            SET keycloak_id = $1, updated_at = NOW()
            WHERE registration_id = $2
            "#,
            keycloak_id,
            registration_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn mark_verified(&self, registration_id: &str, user_id: &str) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE user_registrations
            SET status = 'verified', verified_at = NOW(), user_id = $1, updated_at = NOW()
            WHERE registration_id = $2
            "#,
            user_id,
            registration_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_expired(&self) -> Result<Vec<UserRegistration>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT registration_id, email, username, first_name, last_name, phone,
                   verification_token, status, keycloak_id, user_id, expires_at,
                   verified_at, created_at, updated_at
            FROM user_registrations
            WHERE status = 'pending' AND expires_at < NOW()
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                use chrono::TimeZone;
                UserRegistration {
                    registration_id: r.get("registration_id"),
                    email: r.get("email"),
                    username: r.get("username"),
                    first_name: r.get("first_name"),
                    last_name: r.get("last_name"),
                    phone: r.get("phone"),
                    verification_token: r.get("verification_token"),
                    status: parse_status(r.get("status")),
                    keycloak_id: r.get("keycloak_id"),
                    user_id: r.get("user_id"),
                    expires_at: Utc.from_utc_datetime(&r.get("expires_at")),
                    verified_at: r
                        .get::<Option<chrono::NaiveDateTime>, _>("verified_at")
                        .map(|dt| Utc.from_utc_datetime(&dt)),
                    created_at: Utc.from_utc_datetime(&r.get("created_at")),
                    updated_at: Utc.from_utc_datetime(&r.get("updated_at")),
                }
            })
            .collect())
    }

    async fn mark_expired(&self, registration_ids: Vec<String>) -> Result<(), AppError> {
        if registration_ids.is_empty() {
            return Ok(());
        }

        sqlx::query!(
            r#"
            UPDATE user_registrations
            SET status = 'expired', updated_at = NOW()
            WHERE registration_id = ANY($1)
            "#,
            &registration_ids
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(&self, user: &User) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO users 
            (user_id, keycloak_id, email, username, first_name, last_name, phone,
             photo, is_verified, role, source, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
            user.user_id,
            user.keycloak_id,
            user.email,
            user.username,
            user.first_name,
            user.last_name,
            user.phone,
            user.photo,
            user.is_verified,
            user.role.to_string(),
            user.source.to_string(),
            user.is_active,
            user.created_at.naive_utc(),
            user.updated_at.naive_utc(),
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT user_id, keycloak_id, email, username, first_name, last_name, phone,
                   photo, is_verified, role, source, is_active, created_at, updated_at
            FROM users
            WHERE LOWER(email) = LOWER($1)
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            use chrono::TimeZone;
            User {
                user_id: r.get("user_id"),
                keycloak_id: r.get("keycloak_id"),
                email: r.get("email"),
                username: r.get("username"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                phone: r.get("phone"),
                photo: r.get("photo"),
                is_verified: r.get("is_verified"),
                role: parse_role(r.get("role")),
                source: parse_source(r.get("source")),
                is_active: r.get("is_active"),
                created_at: Utc.from_utc_datetime(&r.get("created_at")),
                updated_at: Utc.from_utc_datetime(&r.get("updated_at")),
            }
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT user_id, keycloak_id, email, username, first_name, last_name, phone,
                   photo, is_verified, role, source, is_active, created_at, updated_at
            FROM users
            WHERE LOWER(username) = LOWER($1)
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            use chrono::TimeZone;
            User {
                user_id: r.get("user_id"),
                keycloak_id: r.get("keycloak_id"),
                email: r.get("email"),
                username: r.get("username"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                phone: r.get("phone"),
                photo: r.get("photo"),
                is_verified: r.get("is_verified"),
                role: parse_role(r.get("role")),
                source: parse_source(r.get("source")),
                is_active: r.get("is_active"),
                created_at: Utc.from_utc_datetime(&r.get("created_at")),
                updated_at: Utc.from_utc_datetime(&r.get("updated_at")),
            }
        }))
    }
}

fn parse_status(s: String) -> RegistrationStatus {
    match s.as_str() {
        "pending" => RegistrationStatus::Pending,
        "verified" => RegistrationStatus::Verified,
        "expired" => RegistrationStatus::Expired,
        "cancelled" => RegistrationStatus::Cancelled,
        _ => RegistrationStatus::Pending,
    }
}

fn parse_role(s: String) -> UserRole {
    match s.as_str() {
        "admin" => UserRole::Admin,
        "partner" => UserRole::Partner,
        "operator" => UserRole::Operator,
        _ => UserRole::User,
    }
}

fn parse_source(s: String) -> UserSource {
    match s.as_str() {
        "internal" => UserSource::Internal,
        _ => UserSource::Web,
    }
}

pub async fn create_user_preferences(pool: &PgPool, user_id: &str) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO user_preferences (user_id, language, timezone, notifications_enabled, theme)
        VALUES ($1, 'en', 'UTC', true, 'light')
        "#,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn log_keycloak_sync(
    pool: &PgPool,
    user_id: Option<&str>,
    keycloak_id: Option<&str>,
    action: &str,
    status: &str,
    details: Option<&str>,
    error_message: Option<&str>,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO keycloak_sync_log (user_id, keycloak_id, action, status, details, error_message)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        user_id,
        keycloak_id,
        action,
        status,
        details,
        error_message,
    )
    .execute(pool)
    .await?;

    Ok(())
}
