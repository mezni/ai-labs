use async_trait::async_trait;
use sqlx::Row;
use crate::domain::entities::network::Network;
use crate::domain::value_objects::{NetworkId, NetworkType, Email, PhoneNumber, Address};
use crate::domain::repositories::NetworkRepository;
use crate::infrastructure::db::postgres::PostgresManager;
use crate::core::errors::ServiceError;
use chrono::NaiveDateTime;

pub struct NetworkRepositoryPostgres {
    db: PostgresManager,
}

impl NetworkRepositoryPostgres {
    pub fn new(db: PostgresManager) -> Self {
        Self { db }
    }

    fn map_type(s: &str) -> NetworkType {
        match s {
            "individual" => NetworkType::Individual,
            "company" => NetworkType::Company,
            _ => NetworkType::Individual,
        }
    }
}
#[async_trait]
impl NetworkRepository for NetworkRepositoryPostgres {
    async fn get_by_id(&self, id: &NetworkId) -> Result<Option<Network>, AppError> {
        let row = sqlx::query("SELECT * FROM networks WHERE network_id = $1")
            .bind(id.0)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| AppError::internal(&format!("DB error: {}", e)))?;

        if let Some(row) = row {
            Ok(Some(Self::map_row_to_network(row)?))
        } else {
            Ok(None)
        }
    }

    async fn get_by_name(&self, name: &str) -> Result<Option<Network>, AppError> {
        let row = sqlx::query("SELECT * FROM networks WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| AppError::internal(&format!("DB error: {}", e)))?;

        if let Some(row) = row {
            Ok(Some(Self::map_row_to_network(row)?))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, network: &Network) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO networks (name, type, contact_email, phone_number, address, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(network.name())
        .bind(match network.network_type() {
            NetworkType::Individual => "individual",
            NetworkType::Company => "company",
        })
        .bind(network.contact_email().map(|e| e.value()))
        .bind(network.phone_number().map(|p| p.value()))
        .bind(network.address().map(|a| a.value()))
        .bind(network.created_by())
        .bind(network.created_at())
        .bind(network.updated_at())
        .execute(&self.db.pool)
        .await
        .map_err(|e| AppError::internal(&format!("DB error: {}", e)))?;

        Ok(())
    }

    async fn update(&self, network: &Network) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE networks
             SET name = $1, type = $2, contact_email = $3, phone_number = $4, address = $5,
                 updated_by = $6, updated_at = $7
             WHERE network_id = $8"
        )
        .bind(network.name())
        .bind(match network.network_type() {
            NetworkType::Individual => "individual",
            NetworkType::Company => "company",
        })
        .bind(network.contact_email().map(|e| e.value()))
        .bind(network.phone_number().map(|p| p.value()))
        .bind(network.address().map(|a| a.value()))
        .bind(network.updated_by())
        .bind(network.updated_at())
        .bind(network.id().0)
        .execute(&self.db.pool)
        .await
        .map_err(|e| AppError::internal(&format!("DB error: {}", e)))?;

        Ok(())
    }

    async fn delete(&self, id: &NetworkId) -> Result<(), AppError> {
        sqlx::query("DELETE FROM networks WHERE network_id = $1")
            .bind(id.0)
            .execute(&self.db.pool)
            .await
            .map_err(|e| AppError::internal(&format!("DB error: {}", e)))?;

        Ok(())
    }

    /// New method: get all networks
    async fn get_all(&self) -> Result<Vec<Network>, AppError> {
        let rows = sqlx::query("SELECT * FROM networks")
            .fetch_all(&self.db.pool)
            .await
            .map_err(|e| AppError::internal(&format!("DB error: {}", e)))?;

        let mut networks = Vec::new();
        for row in rows {
            networks.push(Self::map_row_to_network(row)?);
        }

        Ok(networks)
    }
}

impl NetworkRepositoryPostgres {
    /// Helper to map a SQL row into a Network entity
    fn map_row_to_network(row: sqlx::postgres::PgRow) -> Result<Network, AppError> {
        let network_type_str: String = row.try_get("type").map_err(|e| AppError::internal(&format!("DB error: {}", e)))?;
        let network_type = match network_type_str.as_str() {
            "individual" => NetworkType::Individual,
            "company" => NetworkType::Company,
            _ => NetworkType::Individual,
        };

        let contact_email: Option<String> = row.try_get("contact_email").ok();
        let phone_number: Option<String> = row.try_get("phone_number").ok();
        let address: Option<String> = row.try_get("address").ok();

        Ok(Network::new(
            NetworkId(row.try_get("network_id").map_err(|e| AppError::internal(&format!("DB error: {}", e)))?),
            row.try_get("name").map_err(|e| AppError::internal(&format!("DB error: {}", e)))?,
            network_type,
            contact_email.and_then(|e| Email::new(e).ok()),
            phone_number.and_then(|p| PhoneNumber::new(p).ok()),
            address.and_then(|a| Address::new(a).ok()),
            row.try_get("created_by").map_err(|e| AppError::internal(&format!("DB error: {}", e)))?,
        ))
    }
}
