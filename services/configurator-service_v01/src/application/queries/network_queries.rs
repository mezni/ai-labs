use crate::domain::{DomainError, Network, NetworkRepository};

// ======================== GET NETWORK ========================
pub struct GetNetworkQuery {
    pub id: i32,
}

pub struct GetNetworkHandler<R: NetworkRepository> {
    pub repo: R,
}

impl<R: NetworkRepository + Send + Sync> GetNetworkHandler<R> {
    pub async fn execute(&self, query: GetNetworkQuery) -> Result<Network, DomainError> {
        self.repo
            .find_by_id(query.id.into())
            .await?
            .ok_or(DomainError::NetworkNotFound(query.id))
    }
}

// ======================== LIST NETWORKS ========================
pub struct ListNetworksQuery {
    pub page: u32,
    pub page_size: u32,
}

pub struct ListNetworksHandler<R: NetworkRepository> {
    pub repo: R,
}

impl<R: NetworkRepository + Send + Sync> ListNetworksHandler<R> {
    pub async fn execute(&self, query: ListNetworksQuery) -> Result<Vec<Network>, DomainError> {
        self.repo.list_all(query.page, query.page_size).await
    }
}
