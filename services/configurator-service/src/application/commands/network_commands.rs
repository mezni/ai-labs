use crate::domain::{Network, NetworkRepository, DomainError, NetworkType, ContactInfo};
use crate::application::dtos::{CreateNetworkRequest, UpdateNetworkRequest};

// ======================== CREATE NETWORK ========================
pub struct CreateNetworkCommand {
    pub request: CreateNetworkRequest,
}

pub struct CreateNetworkHandler<R: NetworkRepository> {
    pub repo: R,
}

impl<R: NetworkRepository + Send + Sync> CreateNetworkHandler<R> {
    pub async fn execute(&self, command: CreateNetworkCommand) -> Result<Network, DomainError> {
        let network_type = NetworkType::from_str(&command.request.network_type)?;
        let contact_info = ContactInfo::new(
            command.request.email,
            command.request.phone,
            command.request.address,
        )?;

        let network = Network::new(
            command.request.name,
            network_type,
            contact_info,
            command.request.created_by,
        )?;

        self.repo.save(&network).await
    }
}

// ======================== UPDATE NETWORK ========================
pub struct UpdateNetworkCommand {
    pub id: i32,
    pub request: UpdateNetworkRequest,
}

pub struct UpdateNetworkHandler<R: NetworkRepository> {
    pub repo: R,
}

impl<R: NetworkRepository + Send + Sync> UpdateNetworkHandler<R> {
    pub async fn execute(&self, command: UpdateNetworkCommand) -> Result<Network, DomainError> {
        let mut network = self
            .repo
            .find_by_id(command.id.into())
            .await?
            .ok_or(DomainError::NetworkNotFound(command.id))?;

        let network_type = match command.request.network_type {
            Some(ref s) => Some(NetworkType::from_str(s)?),
            None => None,
        };

        let contact_info = Some(ContactInfo::new(
            command.request.email,
            command.request.phone,
            command.request.address,
        )?);

        network.update(
            command.request.name,
            network_type,
            contact_info,
            command.request.updated_by,
        )?;

        self.repo.update(&network).await
    }
}

// ======================== DELETE NETWORK ========================
pub struct DeleteNetworkCommand {
    pub id: i32,
    pub deleted_by: String,
}

pub struct DeleteNetworkHandler<R: NetworkRepository> {
    pub repo: R,
}

impl<R: NetworkRepository + Send + Sync> DeleteNetworkHandler<R> {
    pub async fn execute(&self, command: DeleteNetworkCommand) -> Result<(), DomainError> {
        // Optional: could mark deleted_by somewhere if needed
        self.repo.delete(command.id.into()).await
    }
}
