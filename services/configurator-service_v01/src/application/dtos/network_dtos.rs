use crate::domain::Network;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ======================== CREATE NETWORK REQUEST ========================
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateNetworkRequest {
    pub name: String,
    pub network_type: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_by: String,
}

// ======================== UPDATE NETWORK REQUEST ========================
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateNetworkRequest {
    pub name: Option<String>,
    pub network_type: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub updated_by: String,
}

// ======================== NETWORK RESPONSE ========================
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct NetworkResponse {
    pub id: i32,
    pub name: String,
    pub network_type: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

impl From<Network> for NetworkResponse {
    fn from(network: Network) -> Self {
        Self {
            id: network.id.0,
            name: network.name,
            network_type: network.network_type.to_string(),
            email: network.contact_info.email,
            phone: network.contact_info.phone,
            address: network.contact_info.address,
        }
    }
}
