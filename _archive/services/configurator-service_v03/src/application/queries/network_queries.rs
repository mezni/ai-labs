use crate::application::dtos::{NetworkListDto, NetworkQueryDto, NetworkResponseDto};
use crate::core::AppError;
use crate::domain::network::NetworkRepository;
use crate::infrastructure::repositories::{Pagination, QueryFilters};
use async_trait::async_trait;
use tracing::info;

#[async_trait]
pub trait NetworkQueryHandler: Send + Sync {
    async fn get_network_by_id(&self, id: i32) -> Result<Option<NetworkResponseDto>, AppError>;
    async fn get_network_by_name(&self, name: &str)
    -> Result<Option<NetworkResponseDto>, AppError>;
    async fn list_networks(&self, query: NetworkQueryDto) -> Result<NetworkListDto, AppError>;
    async fn get_all_networks(&self) -> Result<Vec<NetworkResponseDto>, AppError>;
}

pub struct NetworkQueryHandlerImpl<T: NetworkRepository> {
    repository: T,
}

impl<T: NetworkRepository> NetworkQueryHandlerImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: NetworkRepository> NetworkQueryHandler for NetworkQueryHandlerImpl<T> {
    async fn get_network_by_id(&self, id: i32) -> Result<Option<NetworkResponseDto>, AppError> {
        let network_id = crate::domain::value_objects::NetworkId(id);

        let network = self.repository.get_by_id(&network_id).await?;
        Ok(network.as_ref().map(NetworkResponseDto::from))
    }

    async fn get_network_by_name(
        &self,
        name: &str,
    ) -> Result<Option<NetworkResponseDto>, AppError> {
        let network = self.repository.get_by_name(name).await?;
        Ok(network.as_ref().map(NetworkResponseDto::from))
    }

    async fn list_networks(&self, query: NetworkQueryDto) -> Result<NetworkListDto, AppError> {
        info!("Listing networks with query: {:?}", query);

        // For now, get all networks and filter in memory
        // In a real application, you would implement database-level filtering
        let all_networks = self.repository.get_all().await?;

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
        let per_page = query.per_page.unwrap_or(20).min(100); // Cap at 100
        let total = filtered_networks.len();
        let total_pages = (total as f32 / per_page as f32).ceil() as u32;

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

    async fn get_all_networks(&self) -> Result<Vec<NetworkResponseDto>, AppError> {
        let networks = self.repository.get_all().await?;
        let response_dtos: Vec<NetworkResponseDto> =
            networks.iter().map(NetworkResponseDto::from).collect();
        Ok(response_dtos)
    }
}
