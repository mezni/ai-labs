use crate::domain::network::{Network, NetworkId, NetworkRepository};
//use crate::application::dtos::network_dtos::{CreateNetworkDto, UpdateNetworkDto};
use async_trait::async_trait;
use std::sync::Arc;

/// The NetworkService is the application service layer.
/// It encapsulates business logic and talks to the repository.
#[derive(Clone)]
pub struct NetworkService<R: NetworkRepository> {
    repo: Arc<R>,
}

impl<R: NetworkRepository> NetworkService<R> {
    /// Create a new NetworkService with a repository
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    /// Create a new network
    pub async fn create_network(&self, dto: CreateNetworkDto) -> anyhow::Result<Network> {
        let network = Network::new_from_dto(&dto);

        // Persist in repository
        self.repo.create(&network).await?;

        Ok(network)
    }

    /// Update an existing network
    pub async fn update_network(&self, id: NetworkId, dto: UpdateNetworkDto) -> anyhow::Result<Network> {
        let mut network = self
            .repo
            .get_by_id(&id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Network not found"))?;

        network.update_from_dto(&dto);

        self.repo.update(&network).await?;

        Ok(network)
    }

    /// Get a network by ID
    pub async fn get_network_by_id(&self, id: NetworkId) -> anyhow::Result<Option<Network>> {
        self.repo.get_by_id(&id).await
    }

    /// Get a network by name
    pub async fn get_network_by_name(&self, name: String) -> anyhow::Result<Option<Network>> {
        self.repo.get_by_name(&name).await
    }

    /// List all networks
    pub async fn list_networks(&self) -> anyhow::Result<Vec<Network>> {
        self.repo.get_all().await
    }

    /// Delete a network
    pub async fn delete_network(&self, id: NetworkId) -> anyhow::Result<()> {
        self.repo.delete(&id).await
    }
}
