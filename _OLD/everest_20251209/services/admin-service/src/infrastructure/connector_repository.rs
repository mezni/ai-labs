use crate::{
    domain::{entities::Connector, repositories::ConnectorRepositoryTrait},
    infrastructure::error::{AppError, AppResult},
};
use bigdecimal::BigDecimal;
use sqlx::PgPool;
use std::str::FromStr;

#[derive(Clone)]
pub struct ConnectorRepository {
    pool: PgPool,
}

impl ConnectorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ConnectorRepositoryTrait for ConnectorRepository {
    async fn create(
        &self,
        connector_id: String,
        station_id: String,
        connector_type_id: i64,
        status_id: i64,
        current_type_id: i64,
        power_kw: Option<f64>,
        voltage: Option<i32>,
        amperage: Option<i32>,
        count_available: Option<i32>,
        count_total: Option<i32>,
        created_by: String,
    ) -> AppResult<Connector> {
        let power_bd = power_kw.map(|p| BigDecimal::from_str(&p.to_string()).unwrap());

        let connector = sqlx::query_as::<_, Connector>(
            r#"
            INSERT INTO connectors (
                connector_id, station_id, connector_type_id, status_id, current_type_id,
                power_kw, voltage, amperage, count_available, count_total, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(&connector_id)
        .bind(&station_id)
        .bind(connector_type_id)
        .bind(status_id)
        .bind(current_type_id)
        .bind(power_bd)
        .bind(voltage)
        .bind(amperage)
        .bind(count_available)
        .bind(count_total)
        .bind(&created_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(connector)
    }

    async fn find_by_id(&self, connector_id: &str) -> AppResult<Option<Connector>> {
        let connector = sqlx::query_as::<_, Connector>(
            r#"
            SELECT * FROM connectors WHERE connector_id = $1
            "#,
        )
        .bind(connector_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(connector)
    }

    async fn find_by_station(&self, station_id: &str) -> AppResult<Vec<Connector>> {
        let connectors = sqlx::query_as::<_, Connector>(
            r#"
            SELECT * FROM connectors 
            WHERE station_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(station_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(connectors)
    }

    async fn update(
        &self,
        connector_id: &str,
        status_id: Option<i64>,
        power_kw: Option<f64>,
        voltage: Option<i32>,
        amperage: Option<i32>,
        count_available: Option<i32>,
        count_total: Option<i32>,
        updated_by: String,
    ) -> AppResult<Connector> {
        let existing = self.find_by_id(connector_id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!(
                "Connector with id {} not found",
                connector_id
            )));
        }

        let power_bd = power_kw.map(|p| BigDecimal::from_str(&p.to_string()).unwrap());

        let connector = sqlx::query_as::<_, Connector>(
            r#"
            UPDATE connectors
            SET 
                status_id = COALESCE($2, status_id),
                power_kw = COALESCE($3, power_kw),
                voltage = COALESCE($4, voltage),
                amperage = COALESCE($5, amperage),
                count_available = COALESCE($6, count_available),
                count_total = COALESCE($7, count_total),
                updated_by = $8,
                updated_at = NOW()
            WHERE connector_id = $1
            RETURNING *
            "#,
        )
        .bind(connector_id)
        .bind(status_id)
        .bind(power_bd)
        .bind(voltage)
        .bind(amperage)
        .bind(count_available)
        .bind(count_total)
        .bind(&updated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(connector)
    }

    async fn delete(&self, connector_id: &str) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM connectors WHERE connector_id = $1
            "#,
        )
        .bind(connector_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Connector with id {} not found",
                connector_id
            )));
        }

        Ok(())
    }
}
