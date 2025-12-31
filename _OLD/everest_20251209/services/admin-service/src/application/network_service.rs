use crate::{
    application::dto::{CreateNetworkRequest, NetworkResponse, UpdateNetworkRequest},
    domain::repositories::NetworkRepositoryTrait,
    infrastructure::{
        NetworkRepository,
        error::{AppError, AppResult},
    },
    utils::id_generator::generate_network_id,
};

pub struct NetworkService {
    repository: NetworkRepository,
}

impl NetworkService {
    pub fn new(repository: NetworkRepository) -> Self {
        Self { repository }
    }

    pub async fn create_network(
        &self,
        req: CreateNetworkRequest,
        created_by: String,
    ) -> AppResult<NetworkResponse> {
        // Validate network_type
        if req.network_type != "INDIVIDUAL" && req.network_type != "COMPANY" {
            return Err(AppError::ValidationError(
                "network_type must be INDIVIDUAL or COMPANY".to_string(),
            ));
        }

        let network_id = generate_network_id();

        let network = self
            .repository
            .create(
                network_id,
                req.name,
                req.network_type,
                req.support_phone,
                req.support_email,
                created_by,
            )
            .await?;

        Ok(NetworkResponse {
            network_id: network.network_id,
            name: network.name,
            network_type: network.network_type,
            support_phone: network.support_phone,
            support_email: network.support_email,
            is_verified: network.is_verified.unwrap_or(false),
            created_at: network.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: network.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: network.created_by,
            updated_by: network.updated_by,
        })
    }

    pub async fn get_network(&self, network_id: &str) -> AppResult<NetworkResponse> {
        let network = self
            .repository
            .find_by_id(network_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Network {} not found", network_id)))?;

        Ok(NetworkResponse {
            network_id: network.network_id,
            name: network.name,
            network_type: network.network_type,
            support_phone: network.support_phone,
            support_email: network.support_email,
            is_verified: network.is_verified.unwrap_or(false),
            created_at: network.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: network.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: network.created_by,
            updated_by: network.updated_by,
        })
    }

    pub async fn list_networks(&self, page: i64, limit: i64) -> AppResult<Vec<NetworkResponse>> {
        let offset = (page - 1) * limit;
        let networks = self.repository.find_all(limit, offset).await?;

        Ok(networks
            .into_iter()
            .map(|n| NetworkResponse {
                network_id: n.network_id,
                name: n.name,
                network_type: n.network_type,
                support_phone: n.support_phone,
                support_email: n.support_email,
                is_verified: n.is_verified.unwrap_or(false),
                created_at: n.created_at.map(|dt| dt.to_rfc3339()),
                updated_at: n.updated_at.map(|dt| dt.to_rfc3339()),
                created_by: n.created_by,
                updated_by: n.updated_by,
            })
            .collect())
    }

    pub async fn update_network(
        &self,
        network_id: &str,
        req: UpdateNetworkRequest,
        updated_by: String,
    ) -> AppResult<NetworkResponse> {
        if let Some(ref network_type) = req.network_type {
            if network_type != "INDIVIDUAL" && network_type != "COMPANY" {
                return Err(AppError::ValidationError(
                    "network_type must be INDIVIDUAL or COMPANY".to_string(),
                ));
            }
        }

        let network = self
            .repository
            .update(
                network_id,
                req.name,
                req.network_type,
                req.support_phone,
                req.support_email,
                req.is_verified,
                updated_by,
            )
            .await?;

        Ok(NetworkResponse {
            network_id: network.network_id,
            name: network.name,
            network_type: network.network_type,
            support_phone: network.support_phone,
            support_email: network.support_email,
            is_verified: network.is_verified.unwrap_or(false),
            created_at: network.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: network.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: network.created_by,
            updated_by: network.updated_by,
        })
    }

    pub async fn delete_network(&self, network_id: &str) -> AppResult<()> {
        self.repository.delete(network_id).await
    }
}
