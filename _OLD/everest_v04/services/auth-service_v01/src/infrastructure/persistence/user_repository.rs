use crate::core::{database::DbPool, errors::AppError};
use crate::domain::{
    entities::User,
    enums::{Source, UserRole},
    repositories::UserRepository,
};
use async_trait::async_trait;
use chrono::Utc;

pub struct UserRepositoryImpl {
    pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<User, AppError> {
        let row = sqlx::query_as::<_, User>(
                r#"
                INSERT INTO users (
                    user_id, keycloak_id, email, username, first_name, last_name, 
                    phone, photo, is_verified, role, network_id, station_id, 
                    source, is_active, created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                RETURNING *
                "#,
            )
            .bind(&user.user_id)
            .bind(&user.keycloak_id)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.phone)
            .bind(&user.photo)
            .bind(&user.is_verified)
            .bind(&user.role.to_string())
            .bind(&user.network_id)
            .bind(&user.station_id)
            .bind(&user.source.to_string())
            .bind(&user.is_active)
            .bind(&user.created_at)
            .bind(&user.updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, AppError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, AppError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE keycloak_id = $1")
            .bind(keycloak_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn update_last_login(&self, user_id: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET last_login_at = $1, updated_at = $1 WHERE user_id = $2")
            .bind(Utc::now())
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}