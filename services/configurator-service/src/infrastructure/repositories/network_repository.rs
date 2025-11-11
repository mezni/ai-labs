use crate::core::{AppError, Database, logger};
use crate::domain::network::{Network, NetworkId, NetworkRepository};
use crate::domain::value_objects::{Address, Email, NetworkType, PhoneNumber};
use chrono::NaiveDateTime;
use sqlx::FromRow;

const INSERT_QUERY: &str = r#"
    INSERT INTO networks (id, name, network_type, contact_email, phone_number, address, owner_name, created_at, updated_at, created_by)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
"#;

const UPDATE_QUERY: &str = r#"
    UPDATE networks 
    SET name = $2, network_type = $3, contact_email = $4, phone_number = $5, 
        address = $6, owner_name = $7, updated_at = $8
    WHERE id = $1
"#;

const DELETE_QUERY: &str = "DELETE FROM networks WHERE id = $1";

const SELECT_BY_ID_QUERY: &str = r#"
    SELECT id, name, network_type, contact_email, phone_number, address, owner_name, created_at, updated_at, created_by
    FROM networks 
    WHERE id = $1
"#;

const SELECT_BY_NAME_QUERY: &str = r#"
    SELECT id, name, network_type, contact_email, phone_number, address, owner_name, created_at, updated_at, created_by
    FROM networks 
    WHERE name = $1
"#;

const SELECT_ALL_QUERY: &str = r#"
    SELECT id, name, network_type, contact_email, phone_number, address, owner_name, created_at, updated_at, created_by
    FROM networks 
    ORDER BY created_at DESC
"#;

#[derive(Debug, Clone, FromRow)]
struct NetworkRecord {
    pub id: i32,
    pub name: String,
    pub network_type: String,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub owner_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
}

impl From<NetworkRecord> for Network {
    fn from(record: NetworkRecord) -> Self {
        Self {
            id: NetworkId(record.id),
            name: record.name,
            network_type: NetworkType::from_str(&record.network_type)
                .unwrap_or(NetworkType::Individual),
            contact_email: record.contact_email.and_then(|e| Email::new(e).ok()),
            phone_number: record.phone_number.and_then(|p| PhoneNumber::new(p).ok()),
            address: record.address.and_then(|a| Address::new(a).ok()),
            owner_name: record.owner_name,
            created_at: record.created_at,
            updated_at: record.updated_at,
            created_by: record.created_by,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostgresNetworkRepository {
    db: Database,
}

impl PostgresNetworkRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

impl NetworkRepository for PostgresNetworkRepository {
    async fn create(&self, network: &Network) -> anyhow::Result<()> {
        let rows_affected = self.db.pool()
            .execute(INSERT_QUERY, 
                &[&network.id().0, &network.name(), &network.network_type().as_str(), 
                &network.contact_email().as_ref().map(|e| e.value()), 
                &network.phone_number().as_ref().map(|p| p.value()), 
                &network.address().as_ref().map(|a| a.value()), 
                &network.owner_name(), &network.created_at(), 
                &network.updated_at(), &network.created_by()])
            .await?;

        if rows_affected == 0 {
            return Err(AppError::database("Failed to create network").into());
        }

        logger::info!("Network created successfully: {}", network.name());
        Ok(())
    }

    async fn get_by_id(&self, id: &NetworkId) -> anyhow::Result<Option<Network>> {
        let record: Option<NetworkRecord> = sqlx::query_as(SELECT_BY_ID_QUERY)
            .bind(id.0)
            .fetch_optional(self.db.pool())
            .await?;

        Ok(record.map(|r| r.into()))
    }

    async fn get_by_name(&self, name: &str) -> anyhow::Result<Option<Network>> {
        let record: Option<NetworkRecord> = sqlx::query_as(SELECT_BY_NAME_QUERY)
            .bind(name)
            .fetch_optional(self.db.pool())
            .await?;

        Ok(record.map(|r| r.into()))
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Network>> {
        let records: Vec<NetworkRecord> = sqlx::query_as(SELECT_ALL_QUERY)
            .fetch_all(self.db.pool())
            .await?;

        Ok(records.into_iter().map(|r| r.into()).collect())
    }

    async fn update(&self, network: &Network) -> anyhow::Result<()> {
        let rows_affected = sqlx::query(UPDATE_QUERY)
            .bind(network.id().0)
            .bind(network.name())
            .bind(network.network_type().as_str())
            .bind(network.contact_email().as_ref().map(|e| e.value()))
            .bind(network.phone_number().as_ref().map(|p| p.value()))
            .bind(network.address().as_ref().map(|a| a.value()))
            .bind(network.owner_name())
            .bind(network.updated_at())
            .execute(self.db.pool())
            .await?;

        if rows_affected == 0 {
            return Err(AppError::not_found("Network").into());
        }

        logger::info!("Network updated successfully: {}", network.name());
        Ok(())
    }

    async fn delete(&self, id: &NetworkId) -> anyhow::Result<()> {
        let rows_affected = sqlx::query(DELETE_QUERY)
            .bind(id.0)
            .execute(self.db.pool())
            .await?;

        if rows_affected == 0 {
            return Err(AppError::not_found("Network").into());
        }

        logger::info!("Network deleted successfully: {}", id.0);
        Ok(())
    }
}