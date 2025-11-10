use crate::domain::{
    entities::networks::{Network, NetworkType},
    errors::DomainError,
    repositories::networks::NetworkRepository,
    value_objects::{AuditInfo, ContactInfo, NetworkId},
};
use async_trait::async_trait;
use sqlx::{PgPool, Row};

pub struct PostgresNetworkRepository {
    pool: PgPool,
}

impl PostgresNetworkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NetworkRepository for PostgresNetworkRepository {
    async fn find_by_id(&self, id: NetworkId) -> Result<Option<Network>, DomainError> {
        let record = sqlx::query!(
            r#"
            SELECT network_id, name, type as network_type, contact_email, phone_number, address,
                   created_by, updated_by, created_at, updated_at
            FROM networks
            WHERE network_id = $1
            "#,
            id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        match record {
            Some(r) => Ok(Some(map_record_to_network(r)?)),
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Network>, DomainError> {
        let record = sqlx::query!(
            r#"
            SELECT network_id, name, type as network_type, contact_email, phone_number, address,
                   created_by, updated_by, created_at, updated_at
            FROM networks
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        match record {
            Some(r) => Ok(Some(map_record_to_network(r)?)),
            None => Ok(None),
        }
    }

    async fn save(&self, network: &Network) -> Result<Network, DomainError> {
        // Check for duplicate
        if let Some(existing) = self.find_by_name(&network.name).await? {
            if existing.id.0 != network.id.0 {
                return Err(DomainError::DuplicateName(network.name.clone()));
            }
        }

        let record = sqlx::query!(
            r#"
            INSERT INTO networks (name, type, contact_email, phone_number, address, 
                                  created_by, updated_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING network_id, name, type as network_type, contact_email, phone_number, address,
                      created_by, updated_by, created_at, updated_at
            "#,
            network.name,
            network.network_type.as_str(),
            network.contact_info.email,
            network.contact_info.phone,
            network.contact_info.address,
            network.audit_info.created_by,
            network.audit_info.updated_by,
            network.audit_info.created_at,
            network.audit_info.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        map_record_to_network(record)
    }

    async fn update(&self, network: &Network) -> Result<Network, DomainError> {
        if let Some(existing) = self.find_by_name(&network.name).await? {
            if existing.id.0 != network.id.0 {
                return Err(DomainError::DuplicateName(network.name.clone()));
            }
        }

        let record = sqlx::query!(
            r#"
            UPDATE networks
            SET name=$1, type=$2, contact_email=$3, phone_number=$4, address=$5,
                updated_by=$6, updated_at=$7
            WHERE network_id=$8
            RETURNING network_id, name, type as network_type, contact_email, phone_number, address,
                      created_by, updated_by, created_at, updated_at
            "#,
            network.name,
            network.network_type.as_str(),
            network.contact_info.email,
            network.contact_info.phone,
            network.contact_info.address,
            network.audit_info.updated_by,
            network.audit_info.updated_at,
            network.id.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        map_record_to_network(record)
    }

    async fn delete(&self, id: NetworkId) -> Result<(), DomainError> {
        let result = sqlx::query!("DELETE FROM networks WHERE network_id=$1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NetworkNotFound(id.0));
        }

        Ok(())
    }

    async fn list_all(&self, page: u32, page_size: u32) -> Result<Vec<Network>, DomainError> {
        let offset = (page - 1) * page_size;
        let records = sqlx::query!(
            r#"
            SELECT network_id, name, type as network_type, contact_email, phone_number, address,
                   created_by, updated_by, created_at, updated_at
            FROM networks
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        let mut networks = Vec::new();
        for record in records {
            networks.push(map_record_to_network(record)?);
        }

        Ok(networks)
    }
}

fn map_record_to_network(record: sqlx::postgres::PgRow) -> Result<Network, DomainError> {
    let network_type = NetworkType::from_str(&record.try_get("network_type")?)?;
    let contact_info = ContactInfo::new(
        record.try_get("contact_email")?,
        record.try_get("phone_number")?,
        record.try_get("address")?,
    )?;
    let audit_info = AuditInfo {
        created_by: record.try_get("created_by")?,
        updated_by: record.try_get("updated_by")?,
        created_at: record.try_get("created_at")?,
        updated_at: record.try_get("updated_at")?,
    };

    Ok(Network {
        id: NetworkId(record.try_get("network_id")?),
        name: record.try_get("name")?,
        network_type,
        contact_info,
        audit_info,
    })
}
