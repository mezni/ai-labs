// src/application/services/network_application_service.rs
use crate::domain::repositories::network_repository::NetworkRepository;
use crate::domain::entities::network::Network;
use crate::application::dtos::network_dtos::{CreateNetworkDto, UpdateNetworkDto, NetworkDto};

pub struct NetworkApplicationService<R: NetworkRepository> {
    repository: R,
}

impl<R: NetworkRepository> NetworkApplicationService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_network(&self, dto: CreateNetworkDto) -> anyhow::Result<()> {
        let network = Network::new_from_dto(&dto);
        self.repository.create(&network).await
    }

    pub async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<NetworkDto>> {
        let network = self.repository.get_by_id(&id.into()).await?;
        Ok(network.as_ref().map(|n| n.into()))
    }

    pub async fn get_all_networks(&self) -> anyhow::Result<Vec<NetworkDto>> {
        let networks = self.repository.get_all().await?;
        Ok(networks.iter().map(|n| n.into()).collect())
    }

    pub async fn update_network(&self, id: i32, dto: UpdateNetworkDto) -> anyhow::Result<()> {
        if let Some(mut network) = self.repository.get_by_id(&id.into()).await? {
            network.update_from_dto(&dto);
            self.repository.update(&network).await
        } else {
            anyhow::bail!("Network not found")
        }
    }

    pub async fn delete_network(&self, id: i32) -> anyhow::Result<()> {
        self.repository.delete(&id.into()).await
    }
}
