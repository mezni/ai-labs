use crate::domain::network::{NetworkRepository, Network};
use crate::application::dtos::network_dtos::{CreateNetworkDto, UpdateNetworkDto};
use async_trait::async_trait;

pub struct NetworkService<R: NetworkRepository> {
    repo: R,
}

impl<R: NetworkRepository> NetworkService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: NetworkRepository> NetworkService<R> {
    pub async fn create_network(&self, dto: CreateNetworkDto) -> anyhow::Result<Network> {
        let network = Network::new_from_dto(&dto);
        self.repo.create(&network).await?;
        Ok(network)
    }

    pub async fn update_network(&self, id: i32, dto: UpdateNetworkDto) -> anyhow::Result<Network> {
        let mut network = self.repo.get_by_id(&crate::domain::network::NetworkId(id))
            .await?
            .ok_or_else(|| anyhow::anyhow!("Network not found"))?;
        network.update_from_dto(&dto);
        self.repo.update(&network).await?;
        Ok(network)
    }

    pub async fn get_network(&self, id: i32) -> anyhow::Result<Option<Network>> {
        self.repo.get_by_id(&crate::domain::network::NetworkId(id)).await
    }

    pub async fn get_all_networks(&self) -> anyhow::Result<Vec<Network>> {
        self.repo.get_all().await
    }

    pub async fn delete_network(&self, id: i32) -> anyhow::Result<()> {
        self.repo.delete(&crate::domain::network::NetworkId(id)).await
    }
}
