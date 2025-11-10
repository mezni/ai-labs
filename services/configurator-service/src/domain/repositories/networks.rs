use crate::domain::entities::networks::Network;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::NetworkId;
use async_trait::async_trait;

#[async_trait]
pub trait NetworkRepository: Send + Sync {
    async fn find_by_id(&self, id: NetworkId) -> Result<Option<Network>, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Network>, DomainError>;
    async fn save(&self, network: &Network) -> Result<Network, DomainError>;
    async fn update(&self, network: &Network) -> Result<Network, DomainError>;
    async fn delete(&self, id: NetworkId) -> Result<(), DomainError>;
    async fn list_all(&self, page: u32, page_size: u32) -> Result<Vec<Network>, DomainError>;
}

#[async_trait]
pub trait NetworkService: Send + Sync {
    async fn create_network(
        &self,
        name: String,
        network_type: String,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        address: Option<String>,
        created_by: String,
    ) -> Result<Network, DomainError>;

    async fn get_network(&self, id: i32) -> Result<Network, DomainError>;

    async fn update_network(
        &self,
        id: i32,
        name: Option<String>,
        network_type: Option<String>,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        address: Option<String>,
        updated_by: String,
    ) -> Result<Network, DomainError>;

    async fn delete_network(&self, id: i32, deleted_by: String) -> Result<(), DomainError>;

    async fn list_networks(&self, page: u32, page_size: u32) -> Result<Vec<Network>, DomainError>;
}
