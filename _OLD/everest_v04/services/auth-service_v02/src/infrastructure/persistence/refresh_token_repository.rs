use crate::core::{database::DbPool, errors::AppError};
use crate::domain::{entities::RefreshToken, repositories::RefreshTokenRepository};
use async_trait::async_trait;
use chrono::Utc;

pub struct RefreshTokenRepositoryImpl {
    pool: DbPool,
}

impl RefreshTokenRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenRepositoryImpl {
    async fn create(&self, token: &RefreshToken) -> Result<RefreshToken, AppError> {
        let row = sqlx::query_as::<_, RefreshToken>(
            r#"
                INSERT INTO refresh_tokens (
                    token_id, user_id, refresh_token, expires_at, created_at,
                    ip_address, user_agent
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
                "#,
        )
        .bind(&token.token_id)
        .bind(&token.user_id)
        .bind(&token.refresh_token)
        .bind(&token.expires_at)
        .bind(&token.created_at)
        .bind(&token.ip_address)
        .bind(&token.user_agent)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError> {
        let result = sqlx::query_as::<_, RefreshToken>(
            "SELECT * FROM refresh_tokens WHERE refresh_token = $1 AND revoked_at IS NULL",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn revoke(&self, token_id: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE refresh_tokens SET revoked_at = $1 WHERE token_id = $2")
            .bind(Utc::now())
            .bind(token_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, AppError> {
        let result = sqlx::query("DELETE FROM refresh_tokens WHERE expires_at < $1")
            .bind(Utc::now())
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
