// src/domain/repositories/network_repository.rs
use crate::domain::entities::network::Network;
use crate::domain::value_objects::NetworkId;

#[async_trait::async_trait]
pub trait NetworkRepository: Send + Sync {
    async fn create(&self, network: &Network) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: &NetworkId) -> anyhow::Result<Option<Network>>;
    async fn get_all(&self) -> anyhow::Result<Vec<Network>>;
    async fn update(&self, network: &Network) -> anyhow::Result<()>;
    async fn delete(&self, id: &NetworkId) -> anyhow::Result<()>;
}
