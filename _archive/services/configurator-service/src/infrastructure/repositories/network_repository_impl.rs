use crate::domain::network::{Network, NetworkRepository, NetworkId, NetworkType, Email, PhoneNumber, Address};
use async_trait::async_trait;
use sqlx::PgPool;
use chrono::Utc;
use tracing::info;

pub struct NetworkRepositoryImpl {
    pool: PgPool,
}

impl NetworkRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NetworkRepository for NetworkRepositoryImpl {
    async fn create(&self, network: &Network) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO networks
            (name, type, contact_email, phone_number, address, created_by, updated_by, created_at, updated_at)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
            "#,
            network.name,
            network.network_type.as_str(),
            network.contact_email.as_ref().map(|e| e.value()),
            network.phone_number.as_ref().map(|p| p.value()),
            network.address.as_ref().map(|a| a.value()),
            network.created_by,
            network.created_by, // updated_by initially same as created_by
            network.created_at,
            network.updated_at
        )
        .execute(&self.pool)
        .await?;
        info!("Network '{}' created", network.name);
        Ok(())
    }

    async fn get_by_id(&self, id: &NetworkId) -> anyhow::Result<Option<Network>> {
        let row = sqlx::query!(
            r#"
            SELECT network_id, name, type, contact_email, phone_number, address, created_by, updated_by, created_at, updated_at
            FROM networks WHERE network_id = $1
            "#,
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Network {
            id: NetworkId(r.network_id),
            name: r.name,
            network_type: NetworkType::from_str(&r.r#type).unwrap_or(NetworkType::Individual),
            contact_email: r.contact_email.map(|e| Email::new(e).unwrap()),
            phone_number: r.phone_number.map(|p| PhoneNumber::new(p).unwrap()),
            address: r.address.map(|a| Address::new(a).unwrap()),
            owner_name: r.created_by.clone(),
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn get_by_name(&self, name: &str) -> anyhow::Result<Option<Network>> {
        let row = sqlx::query!(
            r#"
            SELECT network_id, name, type, contact_email, phone_number, address, created_by, updated_by, created_at, updated_at
            FROM networks WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Network {
            id: NetworkId(r.network_id),
            name: r.name,
            network_type: NetworkType::from_str(&r.r#type).unwrap_or(NetworkType::Individual),
            contact_email: r.contact_email.map(|e| Email::new(e).unwrap()),
            phone_number: r.phone_number.map(|p| PhoneNumber::new(p).unwrap()),
            address: r.address.map(|a| Address::new(a).unwrap()),
            owner_name: r.created_by.clone(),
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Network>> {
        let rows = sqlx::query!(
            r#"
            SELECT network_id, name, type, contact_email, phone_number, address, created_by, updated_by, created_at, updated_at
            FROM networks
            ORDER BY network_id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Network {
            id: NetworkId(r.network_id),
            name: r.name,
            network_type: NetworkType::from_str(&r.r#type).unwrap_or(NetworkType::Individual),
            contact_email: r.contact_email.map(|e| Email::new(e).unwrap()),
            phone_number: r.phone_number.map(|p| PhoneNumber::new(p).unwrap()),
            address: r.address.map(|a| Address::new(a).unwrap()),
            owner_name: r.created_by.clone(),
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }).collect())
    }

    async fn update(&self, network: &Network) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE networks
            SET name=$1, type=$2, contact_email=$3, phone_number=$4, address=$5, updated_by=$6, updated_at=$7
            WHERE network_id=$8
            "#,
            network.name,
            network.network_type.as_str(),
            network.contact_email.as_ref().map(|e| e.value()),
            network.phone_number.as_ref().map(|p| p.value()),
            network.address.as_ref().map(|a| a.value()),
            network.created_by, // updated_by can be dynamic
            network.updated_at,
            network.id.0
        )
        .execute(&self.pool)
        .await?;
        info!("Network '{}' updated", network.name);
        Ok(())
    }

    async fn delete(&self, id: &NetworkId) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM networks WHERE network_id = $1", id.0)
            .execute(&self.pool)
            .await?;
        info!("Network id={} deleted", id.0);
        Ok(())
    }
}
