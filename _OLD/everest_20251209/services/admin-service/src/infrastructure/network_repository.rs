use crate::{
    domain::{entities::Network, repositories::NetworkRepositoryTrait},
    infrastructure::error::{AppError, AppResult},
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct NetworkRepository {
    pool: PgPool,
}

impl NetworkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NetworkRepositoryTrait for NetworkRepository {
    async fn create(
        &self,
        network_id: String,
        name: String,
        network_type: String,
        support_phone: Option<String>,
        support_email: Option<String>,
        created_by: String,
    ) -> AppResult<Network> {
        let network = sqlx::query_as::<_, Network>(
            r#"
            INSERT INTO networks (network_id, name, network_type, support_phone, support_email, created_by, updated_by)
            VALUES ($1, $2, $3, $4, $5, $6, $6)
            RETURNING *
            "#,
        )
        .bind(&network_id)
        .bind(&name)
        .bind(&network_type)
        .bind(&support_phone)
        .bind(&support_email)
        .bind(&created_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(network)
    }

    async fn find_by_id(&self, network_id: &str) -> AppResult<Option<Network>> {
        let network = sqlx::query_as::<_, Network>(
            r#"
            SELECT * FROM networks WHERE network_id = $1
            "#,
        )
        .bind(network_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(network)
    }

    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<Network>> {
        let networks = sqlx::query_as::<_, Network>(
            r#"
            SELECT * FROM networks
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(networks)
    }

    async fn update(
        &self,
        network_id: &str,
        name: Option<String>,
        network_type: Option<String>,
        support_phone: Option<String>,
        support_email: Option<String>,
        is_verified: Option<bool>,
        updated_by: String,
    ) -> AppResult<Network> {
        let existing = self.find_by_id(network_id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!(
                "Network with id {} not found",
                network_id
            )));
        }

        let network = sqlx::query_as::<_, Network>(
            r#"
            UPDATE networks
            SET 
                name = COALESCE($2, name),
                network_type = COALESCE($3, network_type),
                support_phone = COALESCE($4, support_phone),
                support_email = COALESCE($5, support_email),
                is_verified = COALESCE($6, is_verified),
                updated_by = $7,
                updated_at = NOW()
            WHERE network_id = $1
            RETURNING *
            "#,
        )
        .bind(network_id)
        .bind(&name)
        .bind(&network_type)
        .bind(&support_phone)
        .bind(&support_email)
        .bind(is_verified)
        .bind(&updated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(network)
    }

    async fn delete(&self, network_id: &str) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM networks WHERE network_id = $1
            "#,
        )
        .bind(network_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Network with id {} not found",
                network_id
            )));
        }

        Ok(())
    }
}
