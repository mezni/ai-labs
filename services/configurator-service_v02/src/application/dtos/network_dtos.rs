// src/application/dtos/network_dtos.rs
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::domain::value_objects::NetworkType;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateNetworkDto {
    pub name: String,
    pub network_type: NetworkType,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub owner_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UpdateNetworkDto {
    pub name: String,
    pub network_type: NetworkType,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub owner_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct NetworkDto {
    pub id: i32,
    pub name: String,
    pub network_type: NetworkType,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub owner_name: String,
}

// Conversions
impl From<&crate::domain::entities::network::Network> for NetworkDto {
    fn from(network: &crate::domain::entities::network::Network) -> Self {
        Self {
            id: network.id().0,
            name: network.name().to_string(),
            network_type: network.network_type().clone(),
            contact_email: network.contact_email().clone().map(|e| e.0),
            phone_number: network.phone_number().clone().map(|p| p.0),
            address: network.address().clone().map(|a| a.0),
            owner_name: network.owner_name.clone(),
        }
    }
}
