use serde::Deserialize;
use utoipa::ToSchema;
use crate::domain::value_objects::NetworkType;

/// DTO for creating a network
#[derive(Deserialize, ToSchema, Clone)]
pub struct CreateNetworkDto {
    pub name: String,
    pub owner_name: String,
    pub network_type: NetworkType,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
}

/// DTO for updating a network
#[derive(Deserialize, ToSchema, Clone)]
pub struct UpdateNetworkDto {
    pub name: String,
    pub owner_name: String,
    pub network_type: NetworkType,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
}
