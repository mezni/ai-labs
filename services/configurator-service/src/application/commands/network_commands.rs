use crate::application::dtos::{CreateNetworkDto, UpdateNetworkDto};
use crate::core::AppError;
use crate::domain::network::{Network, NetworkRepository};
use async_trait::async_trait;
use tracing::{error, info};

#[async_trait]
pub trait NetworkCommandHandler: Send + Sync {
    async fn create_network(&self, dto: CreateNetworkDto) -> Result<i32, AppError>;
    async fn update_network(&self, id: i32, dto: UpdateNetworkDto) -> Result<(), AppError>;
    async fn delete_network(&self, id: i32) -> Result<(), AppError>;
}

pub struct NetworkCommandHandlerImpl<T: NetworkRepository> {
    repository: T,
}

impl<T: NetworkRepository> NetworkCommandHandlerImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: NetworkRepository> NetworkCommandHandler for NetworkCommandHandlerImpl<T> {
    async fn create_network(&self, dto: CreateNetworkDto) -> Result<i32, AppError> {
        info!("Creating new network: {}", dto.name);

        // Validate DTO
        let contact_email = dto
            .contact_email
            .map(|email| Email::new(email))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        let phone_number = dto
            .phone_number
            .map(|phone| PhoneNumber::new(phone))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        let address = dto
            .address
            .map(|addr| Address::new(addr))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        // Create domain entity
        let create_dto = crate::application::dtos::network_dtos::CreateNetworkDto {
            name: dto.name,
            network_type: dto.network_type,
            contact_email,
            phone_number,
            address,
            owner_name: dto.owner_name,
        };

        let network = Network::new_from_dto(&create_dto);

        // Check if network with same name already exists
        if let Some(existing) = self.repository.get_by_name(network.name()).await? {
            return Err(AppError::validation(&format!(
                "Network with name '{}' already exists",
                existing.name()
            )));
        }

        // Save to repository
        self.repository.create(&network).await?;

        info!("Network created successfully with ID: {}", network.id().0);
        Ok(network.id().0)
    }

    async fn update_network(&self, id: i32, dto: UpdateNetworkDto) -> Result<(), AppError> {
        info!("Updating network with ID: {}", id);

        let network_id = crate::domain::value_objects::NetworkId(id);

        // Fetch existing network
        let mut network = self
            .repository
            .get_by_id(&network_id)
            .await?
            .ok_or_else(|| AppError::not_found(&format!("Network with ID {}", id)))?;

        // Validate DTO
        let contact_email = dto
            .contact_email
            .map(|email| Email::new(email))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        let phone_number = dto
            .phone_number
            .map(|phone| PhoneNumber::new(phone))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        let address = dto
            .address
            .map(|addr| Address::new(addr))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        // Create update DTO
        let update_dto = crate::application::dtos::network_dtos::UpdateNetworkDto {
            name: dto.name,
            network_type: dto.network_type,
            contact_email,
            phone_number,
            address,
            owner_name: dto.owner_name,
        };

        // Update domain entity
        network.update_from_dto(&update_dto);

        // Check for name conflicts (if name changed)
        if let Some(existing) = self.repository.get_by_name(network.name()).await? {
            if existing.id().0 != id {
                return Err(AppError::validation(&format!(
                    "Network with name '{}' already exists",
                    network.name()
                )));
            }
        }

        // Save updates
        self.repository.update(&network).await?;

        info!("Network updated successfully: {}", id);
        Ok(())
    }

    async fn delete_network(&self, id: i32) -> Result<(), AppError> {
        info!("Deleting network with ID: {}", id);

        let network_id = crate::domain::value_objects::NetworkId(id);

        // Check if network exists
        let network = self
            .repository
            .get_by_id(&network_id)
            .await?
            .ok_or_else(|| AppError::not_found(&format!("Network with ID {}", id)))?;

        // Delete from repository
        self.repository.delete(&network_id).await?;

        info!("Network deleted successfully: {} - {}", id, network.name());
        Ok(())
    }
}
