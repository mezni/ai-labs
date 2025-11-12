use crate::core::errors::AppError;
use crate::core::database::ConnectionManager;
use crate::domain::networks::Network;
use crate::application::dtos::networks::{NetworkCreate, NetworkUpdate};
use tracing::{error, info};
use std::sync::Arc;

#[derive(Clone)]
pub struct NetworkRepository {
    connection_manager: Arc<ConnectionManager>,
}

impl NetworkRepository {
    pub fn new(connection_manager: Arc<ConnectionManager>) -> Self {
        Self { connection_manager }
    }

    pub fn get_pool(&self) -> &sqlx::PgPool {
        self.connection_manager.get_pool()
    }

    pub async fn get_all(&self) -> Result<Vec<Network>, AppError> {
        sqlx::query_as::<_, Network>("SELECT * FROM networks")
            .fetch_all(self.get_pool())
            .await
            .map_err(AppError::Database)
    }

    pub async fn get_by_id(&self, network_id: i32) -> Result<Option<Network>, AppError> {
        sqlx::query_as::<_, Network>("SELECT * FROM networks WHERE network_id = $1")
            .bind(network_id)
            .fetch_optional(self.get_pool())
            .await
            .map_err(AppError::Database)
    }

    pub async fn create(&self, network: NetworkCreate, username: String) -> Result<Network, AppError> {
        let result = sqlx::query_as::<_, Network>(
            r#"
            INSERT INTO networks (name, type, contact_email, phone_number, address, created_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(&network.name)
        .bind(&network.type_)
        .bind(&network.contact_email)
        .bind(&network.phone_number)
        .bind(&network.address)
        .bind(&username)
        .fetch_one(self.get_pool())
        .await;

        match result {
            Ok(network) => {
                info!("Network created successfully: {}", network.network_id);
                Ok(network)
            }
            Err(e) => {
                error!("Failed to create network: {}", e);
                Err(AppError::CreateFailed(e.to_string()))
            }
        }
    }

    pub async fn update(
        &self,
        network_id: i32,
        network: NetworkUpdate,
        username: String,
    ) -> Result<Network, AppError> {
        let result = sqlx::query_as::<_, Network>(
            r#"
            UPDATE networks
            SET name = $1, type = $2, contact_email = $3, phone_number = $4,
                address = $5, updated_by = $6
            WHERE network_id = $7
            RETURNING *
            "#,
        )
        .bind(&network.name)
        .bind(&network.type_)
        .bind(&network.contact_email)
        .bind(&network.phone_number)
        .bind(&network.address)
        .bind(&username)
        .bind(network_id)
        .fetch_one(self.get_pool())
        .await;

        match result {
            Ok(network) => {
                info!("Network updated successfully: {}", network.network_id);
                Ok(network)
            }
            Err(sqlx::Error::RowNotFound) => Err(AppError::not_found("Network")),
            Err(e) => {
                error!("Failed to update network: {}", e);
                Err(AppError::UpdateFailed(e.to_string()))
            }
        }
    }

    pub async fn delete(&self, network_id: i32) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM networks WHERE network_id = $1")
            .bind(network_id)
            .execute(self.get_pool())
            .await
            .map_err(AppError::Database)?;

        if result.rows_affected() > 0 {
            info!("Network deleted successfully: {}", network_id);
            Ok(true)
        } else {
            Err(AppError::not_found("Network"))
        }
    }

    pub async fn exists(&self, network_id: i32) -> Result<bool, AppError> {
        let result = sqlx::query("SELECT 1 FROM networks WHERE network_id = $1")
            .bind(network_id)
            .fetch_optional(self.get_pool())
            .await
            .map_err(AppError::Database)?;

        Ok(result.is_some())
    }
}