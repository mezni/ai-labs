use crate::{
    domain::{entities::Station, repositories::StationRepositoryTrait},
    infrastructure::error::{AppError, AppResult},
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct StationRepository {
    pool: PgPool,
}

impl StationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl StationRepositoryTrait for StationRepository {
    async fn create(
        &self,
        station_id: String,
        osm_id: i64,
        name: String,
        address: Option<String>,
        latitude: f64,
        longitude: f64,
        tags: Option<serde_json::Value>,
        network_id: Option<String>,
        created_by: String,
    ) -> AppResult<Station> {
        let station = sqlx::query_as::<_, Station>(
            r#"
            INSERT INTO stations (station_id, osm_id, name, address, location, tags, network_id, created_by)
            VALUES ($1, $2, $3, $4, ST_Point($5, $6)::GEOGRAPHY, $7, $8, $9)
            RETURNING 
                station_id, osm_id, name, address,
                ST_Y(location::geometry) as latitude,
                ST_X(location::geometry) as longitude,
                tags, network_id, created_by, created_at, updated_by, updated_at
            "#,
        )
        .bind(&station_id)
        .bind(osm_id)
        .bind(&name)
        .bind(&address)
        .bind(longitude)
        .bind(latitude)
        .bind(&tags)
        .bind(&network_id)
        .bind(&created_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(station)
    }

    async fn find_by_id(&self, station_id: &str) -> AppResult<Option<Station>> {
        let station = sqlx::query_as::<_, Station>(
            r#"
            SELECT 
                station_id, osm_id, name, address,
                ST_Y(location::geometry) as latitude,
                ST_X(location::geometry) as longitude,
                tags, network_id, created_by, created_at, updated_by, updated_at
            FROM stations
            WHERE station_id = $1
            "#,
        )
        .bind(station_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(station)
    }

    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<Station>> {
        let stations = sqlx::query_as::<_, Station>(
            r#"
            SELECT 
                station_id, osm_id, name, address,
                ST_Y(location::geometry) as latitude,
                ST_X(location::geometry) as longitude,
                tags, network_id, created_by, created_at, updated_by, updated_at
            FROM stations
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(stations)
    }

    async fn find_by_network(&self, network_id: &str) -> AppResult<Vec<Station>> {
        let stations = sqlx::query_as::<_, Station>(
            r#"
            SELECT 
                station_id, osm_id, name, address,
                ST_Y(location::geometry) as latitude,
                ST_X(location::geometry) as longitude,
                tags, network_id, created_by, created_at, updated_by, updated_at
            FROM stations
            WHERE network_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(network_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(stations)
    }

    async fn update(
        &self,
        station_id: &str,
        name: Option<String>,
        address: Option<String>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        tags: Option<serde_json::Value>,
        network_id: Option<String>,
        updated_by: String,
    ) -> AppResult<Station> {
        let existing = self.find_by_id(station_id).await?;
        if existing.is_none() {
            return Err(AppError::NotFound(format!(
                "Station with id {} not found",
                station_id
            )));
        }

        // Update location if both lat/lon provided
        let station = if let (Some(lat), Some(lon)) = (latitude, longitude) {
            sqlx::query_as::<_, Station>(
                r#"
                UPDATE stations
                SET 
                    name = COALESCE($2, name),
                    address = COALESCE($3, address),
                    location = ST_Point($4, $5)::GEOGRAPHY,
                    tags = COALESCE($6, tags),
                    network_id = COALESCE($7, network_id),
                    updated_by = $8,
                    updated_at = NOW()
                WHERE station_id = $1
                RETURNING 
                    station_id, osm_id, name, address,
                    ST_Y(location::geometry) as latitude,
                    ST_X(location::geometry) as longitude,
                    tags, network_id, created_by, created_at, updated_by, updated_at
                "#,
            )
            .bind(station_id)
            .bind(&name)
            .bind(&address)
            .bind(lon)
            .bind(lat)
            .bind(&tags)
            .bind(&network_id)
            .bind(&updated_by)
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Station>(
                r#"
                UPDATE stations
                SET 
                    name = COALESCE($2, name),
                    address = COALESCE($3, address),
                    tags = COALESCE($4, tags),
                    network_id = COALESCE($5, network_id),
                    updated_by = $6,
                    updated_at = NOW()
                WHERE station_id = $1
                RETURNING 
                    station_id, osm_id, name, address,
                    ST_Y(location::geometry) as latitude,
                    ST_X(location::geometry) as longitude,
                    tags, network_id, created_by, created_at, updated_by, updated_at
                "#,
            )
            .bind(station_id)
            .bind(&name)
            .bind(&address)
            .bind(&tags)
            .bind(&network_id)
            .bind(&updated_by)
            .fetch_one(&self.pool)
            .await?
        };

        Ok(station)
    }

    async fn delete(&self, station_id: &str) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM stations WHERE station_id = $1
            "#,
        )
        .bind(station_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Station with id {} not found",
                station_id
            )));
        }

        Ok(())
    }
}
