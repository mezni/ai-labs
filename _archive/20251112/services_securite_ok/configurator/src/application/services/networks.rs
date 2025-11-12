use crate::application::dtos::networks::{NetworkCreate, NetworkUpdate};
use crate::core::database::PoolStats;
use crate::core::errors::AppError;
use crate::domain::networks::Network;
use crate::infrastructure::repositories::networks::NetworkRepository;
use tracing::{error, info};

#[derive(Clone)]
pub struct NetworkService {
    repository: NetworkRepository,
}

impl NetworkService {
    pub fn new(repository: NetworkRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Network>, AppError> {
        self.repository.get_all().await
    }

    pub async fn get_by_id(&self, network_id: i32) -> Result<Option<Network>, AppError> {
        self.repository.get_by_id(network_id).await
    }

    pub async fn create(
        &self,
        network: NetworkCreate,
        username: String,
    ) -> Result<Network, AppError> {
        // Validate network type
        if network.type_ != "individual" && network.type_ != "company" {
            return Err(AppError::validation(
                "Network type must be either 'individual' or 'company'",
            ));
        }

        // Validate email if provided
        if let Some(ref email) = network.contact_email {
            if !is_valid_email(email) {
                return Err(AppError::validation("Invalid email format"));
            }
        }

        self.repository.create(network, username).await
    }

    pub async fn update(
        &self,
        network_id: i32,
        network: NetworkUpdate,
        username: String,
    ) -> Result<Network, AppError> {
        // Validate network type
        if network.type_ != "individual" && network.type_ != "company" {
            return Err(AppError::validation(
                "Network type must be either 'individual' or 'company'",
            ));
        }

        // Validate email if provided
        if let Some(ref email) = network.contact_email {
            if !is_valid_email(email) {
                return Err(AppError::validation("Invalid email format"));
            }
        }

        self.repository.update(network_id, network, username).await
    }

    pub async fn delete(&self, network_id: i32) -> Result<bool, AppError> {
        self.repository.delete(network_id).await
    }

    // Health check method
    pub async fn health_check(&self) -> Result<(), AppError> {
        // Use the repository's connection for health check
        sqlx::query("SELECT 1")
            .execute(self.repository.get_pool())
            .await
            .map_err(|e| {
                error!("Health check failed: {}", e);
                AppError::PoolError(e.to_string())
            })?;
        Ok(())
    }

    // Get connection pool statistics
    pub async fn get_pool_stats(&self) -> Result<PoolStats, AppError> {
        let pool = self.repository.get_pool();
        let size = pool.size();
        let num_idle = pool.num_idle();
        let num_used = size as u32 - num_idle as u32;

        Ok(PoolStats {
            total_connections: size as u32,
            idle_connections: num_idle as u32,
            used_connections: num_used,
            max_connections: pool.options().get_max_connections(),
        })
    }
}

// Simple email validation function
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local_part = parts[0];
    let domain_part = parts[1];

    // Check local part is not empty
    if local_part.is_empty() {
        return false;
    }

    // Check domain part has at least one dot and valid structure
    let domain_parts: Vec<&str> = domain_part.split('.').collect();
    if domain_parts.len() < 2 {
        return false;
    }

    // Check each domain part is not empty
    for part in domain_parts {
        if part.is_empty() {
            return false;
        }
    }

    // Basic character check
    if email.contains(' ') || email.contains("..") {
        return false;
    }

    true
}
