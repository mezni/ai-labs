use crate::domain::{NetworkRepository, DomainError};
use crate::application::commands::{
    CreateNetworkCommand, CreateNetworkHandler,
    UpdateNetworkCommand, UpdateNetworkHandler,
    DeleteNetworkCommand, DeleteNetworkHandler,
};
use crate::application::queries::{
    GetNetworkQuery, GetNetworkHandler,
    ListNetworksQuery, ListNetworksHandler,
};

pub struct NetworkApplicationService<R: NetworkRepository> {
    pub repo: R,
}

impl<R: NetworkRepository + Send + Sync + Clone> NetworkApplicationService<R> {
    pub async fn create_network(&self, command: CreateNetworkCommand) -> Result<crate::domain::Network, DomainError> {
        let handler = CreateNetworkHandler { repo: self.repo.clone() };
        handler.execute(command).await
    }

    pub async fn update_network(&self, command: UpdateNetworkCommand) -> Result<crate::domain::Network, DomainError> {
        let handler = UpdateNetworkHandler { repo: self.repo.clone() };
        handler.execute(command).await
    }

    pub async fn delete_network(&self, command: DeleteNetworkCommand) -> Result<(), DomainError> {
        let handler = DeleteNetworkHandler { repo: self.repo.clone() };
        handler.execute(command).await
    }

    pub async fn get_network(&self, query: GetNetworkQuery) -> Result<crate::domain::Network, DomainError> {
        let handler = GetNetworkHandler { repo: self.repo.clone() };
        handler.execute(query).await
    }

    pub async fn list_networks(&self, query: ListNetworksQuery) -> Result<Vec<crate::domain::Network>, DomainError> {
        let handler = ListNetworksHandler { repo: self.repo.clone() };
        handler.execute(query).await
    }
}
