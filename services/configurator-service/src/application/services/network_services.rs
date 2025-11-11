use crate::application::dtos::{
    CreateNetworkDto, NetworkListDto, NetworkQueryDto, NetworkResponseDto, UpdateNetworkDto,
};
use crate::core::AppError;
use crate::domain::network::NetworkRepository;
use async_trait::async_trait;
use tracing::{info, instrument};

/// Comprehensive network service that combines commands and queries
/// This provides a unified interface for all network operations
pub struct NetworkService<T: NetworkRepository> {
    repository: T,
}

impl<T: NetworkRepository> NetworkService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait NetworkServiceTrait: Send + Sync {
    // Command operations (Write)
    async fn create_network(&self, dto: CreateNetworkDto) -> Result<NetworkResponseDto, AppError>;
    async fn update_network(
        &self,
        id: i32,
        dto: UpdateNetworkDto,
    ) -> Result<NetworkResponseDto, AppError>;
    async fn delete_network(&self, id: i32) -> Result<(), AppError>;

    // Query operations (Read)
    async fn get_network_by_id(&self, id: i32) -> Result<Option<NetworkResponseDto>, AppError>;
    async fn get_network_by_name(&self, name: &str)
    -> Result<Option<NetworkResponseDto>, AppError>;
    async fn list_networks(&self, query: NetworkQueryDto) -> Result<NetworkListDto, AppError>;
    async fn get_all_networks(&self) -> Result<Vec<NetworkResponseDto>, AppError>;

    // Business operations
    async fn network_exists(&self, name: &str) -> Result<bool, AppError>;
    async fn get_networks_by_type(
        &self,
        network_type: &str,
    ) -> Result<Vec<NetworkResponseDto>, AppError>;
}

#[async_trait]
impl<T: NetworkRepository> NetworkServiceTrait for NetworkService<T> {
    #[instrument(skip(self, dto))]
    async fn create_network(&self, dto: CreateNetworkDto) -> Result<NetworkResponseDto, AppError> {
        info!("Creating new network: {}", dto.name);

        // Validate input data
        let contact_email = dto
            .contact_email
            .map(|email| crate::domain::value_objects::Email::new(email))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        let phone_number = dto
            .phone_number
            .map(|phone| crate::domain::value_objects::PhoneNumber::new(phone))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        let address = dto
            .address
            .map(|addr| crate::domain::value_objects::Address::new(addr))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        // Create domain entity
        let create_dto = crate::application::dtos::network_dtos::CreateNetworkDto {
            name: dto.name.clone(),
            network_type: dto.network_type,
            contact_email,
            phone_number,
            address,
            owner_name: dto.owner_name,
        };

        let network = crate::domain::Network::new_from_dto(&create_dto);

        // Business rule: Check for duplicate network names
        if self.repository.get_by_name(network.name()).await?.is_some() {
            return Err(AppError::validation(&format!(
                "Network with name '{}' already exists",
                network.name()
            )));
        }

        // Persist the network
        self.repository.create(&network).await?;

        info!("Network created successfully with ID: {}", network.id().0);

        // Return the created network
        Ok(NetworkResponseDto::from(&network))
    }

    #[instrument(skip(self, dto))]
    async fn update_network(
        &self,
        id: i32,
        dto: UpdateNetworkDto,
    ) -> Result<NetworkResponseDto, AppError> {
        info!("Updating network with ID: {}", id);

        let network_id = crate::domain::value_objects::NetworkId(id);

        // Fetch existing network
        let mut network = self
            .repository
            .get_by_id(&network_id)
            .await?
            .ok_or_else(|| AppError::not_found(&format!("Network with ID {}", id)))?;

        // Validate input data
        let contact_email = dto
            .contact_email
            .map(|email| crate::domain::value_objects::Email::new(email))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        let phone_number = dto
            .phone_number
            .map(|phone| crate::domain::value_objects::PhoneNumber::new(phone))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        let address = dto
            .address
            .map(|addr| crate::domain::value_objects::Address::new(addr))
            .transpose()
            .map_err(|e| AppError::validation(&e))?;

        // Create update DTO
        let update_dto = crate::application::dtos::network_dtos::UpdateNetworkDto {
            name: dto.name.clone(),
            network_type: dto.network_type,
            contact_email,
            phone_number,
            address,
            owner_name: dto.owner_name,
        };

        // Update domain entity
        network.update_from_dto(&update_dto);

        // Business rule: Check for name conflicts (if name changed)
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

        // Return the updated network
        Ok(NetworkResponseDto::from(&network))
    }

    #[instrument(skip(self))]
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

    #[instrument(skip(self))]
    async fn get_network_by_id(&self, id: i32) -> Result<Option<NetworkResponseDto>, AppError> {
        let network_id = crate::domain::value_objects::NetworkId(id);

        let network = self.repository.get_by_id(&network_id).await?;
        Ok(network.as_ref().map(NetworkResponseDto::from))
    }

    #[instrument(skip(self))]
    async fn get_network_by_name(
        &self,
        name: &str,
    ) -> Result<Option<NetworkResponseDto>, AppError> {
        let network = self.repository.get_by_name(name).await?;
        Ok(network.as_ref().map(NetworkResponseDto::from))
    }

    #[instrument(skip(self, query))]
    async fn list_networks(&self, query: NetworkQueryDto) -> Result<NetworkListDto, AppError> {
        info!("Listing networks with query: {:?}", query);

        // Get all networks from repository
        let all_networks = self.repository.get_all().await?;

        // Apply filters
        let mut filtered_networks: Vec<_> = all_networks
            .into_iter()
            .filter(|network| {
                // Filter by network type if provided
                if let Some(network_type) = &query.network_type {
                    if network.network_type().as_str() != network_type {
                        return false;
                    }
                }

                // Filter by search term if provided
                if let Some(search) = &query.search {
                    let search_lower = search.to_lowercase();
                    network.name().to_lowercase().contains(&search_lower)
                        || network.owner_name().to_lowercase().contains(&search_lower)
                        || network
                            .contact_email()
                            .as_ref()
                            .map(|email| email.value().to_lowercase().contains(&search_lower))
                            .unwrap_or(false)
                } else {
                    true
                }
            })
            .collect();

        // Apply sorting
        if let Some(sort_by) = &query.sort_by {
            match sort_by.as_str() {
                "name" => filtered_networks.sort_by(|a, b| a.name().cmp(b.name())),
                "created_at" => {
                    filtered_networks.sort_by(|a, b| a.created_at().cmp(b.created_at()))
                }
                "updated_at" => {
                    filtered_networks.sort_by(|a, b| a.updated_at().cmp(b.updated_at()))
                }
                "owner_name" => {
                    filtered_networks.sort_by(|a, b| a.owner_name().cmp(b.owner_name()))
                }
                _ => {} // Default no sort
            }

            // Apply sort order
            if query.sort_order.as_deref() == Some("desc") {
                filtered_networks.reverse();
            }
        } else {
            // Default sort by created_at desc
            filtered_networks.sort_by(|a, b| b.created_at().cmp(a.created_at()));
        }

        // Apply pagination
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20).min(100); // Cap at 100 for performance
        let total = filtered_networks.len();
        let total_pages = if total > 0 {
            ((total as f32 / per_page as f32).ceil() as u32).max(1)
        } else {
            0
        };

        let start_index = ((page - 1) * per_page) as usize;
        let end_index = (start_index + per_page as usize).min(total);

        let paginated_networks = if start_index < total {
            filtered_networks[start_index..end_index].to_vec()
        } else {
            Vec::new()
        };

        let response_dtos: Vec<NetworkResponseDto> = paginated_networks
            .iter()
            .map(NetworkResponseDto::from)
            .collect();

        Ok(NetworkListDto {
            networks: response_dtos,
            total,
            page,
            per_page,
            total_pages,
        })
    }

    #[instrument(skip(self))]
    async fn get_all_networks(&self) -> Result<Vec<NetworkResponseDto>, AppError> {
        let networks = self.repository.get_all().await?;
        let response_dtos: Vec<NetworkResponseDto> =
            networks.iter().map(NetworkResponseDto::from).collect();
        Ok(response_dtos)
    }

    #[instrument(skip(self))]
    async fn network_exists(&self, name: &str) -> Result<bool, AppError> {
        let network = self.repository.get_by_name(name).await?;
        Ok(network.is_some())
    }

    #[instrument(skip(self))]
    async fn get_networks_by_type(
        &self,
        network_type: &str,
    ) -> Result<Vec<NetworkResponseDto>, AppError> {
        let query = NetworkQueryDto {
            network_type: Some(network_type.to_string()),
            search: None,
            page: None,
            per_page: None,
            sort_by: None,
            sort_order: None,
        };

        let result = self.list_networks(query).await?;
        Ok(result.networks)
    }
}

// Service factory for creating network services
pub struct NetworkServiceFactory;

impl NetworkServiceFactory {
    pub fn create<T: NetworkRepository>(repository: T) -> impl NetworkServiceTrait {
        NetworkService::new(repository)
    }
}

// Extension traits for additional business operations
#[async_trait]
pub trait NetworkBusinessOperations: NetworkServiceTrait {
    async fn bulk_create_networks(
        &self,
        dtos: Vec<CreateNetworkDto>,
    ) -> Result<Vec<NetworkResponseDto>, AppError> {
        let mut results = Vec::new();

        for dto in dtos {
            match self.create_network(dto).await {
                Ok(network) => results.push(network),
                Err(e) => {
                    tracing::warn!("Failed to create network: {}", e);
                    // Continue with other networks
                }
            }
        }

        Ok(results)
    }

    async fn get_network_statistics(&self) -> Result<NetworkStatistics, AppError> {
        let all_networks = self.get_all_networks().await?;

        let total_networks = all_networks.len();
        let individual_count = all_networks
            .iter()
            .filter(|n| {
                matches!(
                    n.network_type,
                    crate::domain::value_objects::NetworkType::Individual
                )
            })
            .count();
        let company_count = total_networks - individual_count;

        let networks_with_email = all_networks
            .iter()
            .filter(|n| n.contact_email.is_some())
            .count();
        let networks_with_phone = all_networks
            .iter()
            .filter(|n| n.phone_number.is_some())
            .count();
        let networks_with_address = all_networks.iter().filter(|n| n.address.is_some()).count();

        Ok(NetworkStatistics {
            total_networks,
            individual_count,
            company_count,
            networks_with_email,
            networks_with_phone,
            networks_with_address,
        })
    }
}

// Auto-implement for all types that implement NetworkServiceTrait
impl<T: NetworkServiceTrait> NetworkBusinessOperations for T {}

// Statistics DTO
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct NetworkStatistics {
    pub total_networks: usize,
    pub individual_count: usize,
    pub company_count: usize,
    pub networks_with_email: usize,
    pub networks_with_phone: usize,
    pub networks_with_address: usize,
}
