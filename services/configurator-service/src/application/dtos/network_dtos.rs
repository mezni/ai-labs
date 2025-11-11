use crate::domain::value_objects::{Address, Email, NetworkType, PhoneNumber};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateNetworkDto {
    #[schema(example = "My Network")]
    pub name: String,

    #[schema(example = "individual")]
    pub network_type: NetworkType,

    #[schema(example = "contact@example.com")]
    pub contact_email: Option<String>,

    #[schema(example = "+1234567890")]
    pub phone_number: Option<String>,

    #[schema(example = "123 Main St, City, Country")]
    pub address: Option<String>,

    #[schema(example = "John Doe")]
    pub owner_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateNetworkDto {
    #[schema(example = "Updated Network")]
    pub name: String,

    #[schema(example = "company")]
    pub network_type: NetworkType,

    #[schema(example = "updated@example.com")]
    pub contact_email: Option<String>,

    #[schema(example = "+0987654321")]
    pub phone_number: Option<String>,

    #[schema(example = "456 Oak St, City, Country")]
    pub address: Option<String>,

    #[schema(example = "Jane Smith")]
    pub owner_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkResponseDto {
    pub id: i32,

    #[schema(example = "My Network")]
    pub name: String,

    pub network_type: NetworkType,

    #[schema(example = "contact@example.com")]
    pub contact_email: Option<String>,

    #[schema(example = "+1234567890")]
    pub phone_number: Option<String>,

    #[schema(example = "123 Main St, City, Country")]
    pub address: Option<String>,

    #[schema(example = "John Doe")]
    pub owner_name: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkListDto {
    pub networks: Vec<NetworkResponseDto>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkQueryDto {
    #[schema(example = "company")]
    pub network_type: Option<String>,

    #[schema(example = "John")]
    pub search: Option<String>,

    #[schema(example = "1")]
    pub page: Option<u32>,

    #[schema(example = "20")]
    pub per_page: Option<u32>,

    #[schema(example = "created_at")]
    pub sort_by: Option<String>,

    #[schema(example = "desc")]
    pub sort_order: Option<String>,
}

impl From<&crate::domain::Network> for NetworkResponseDto {
    fn from(network: &crate::domain::Network) -> Self {
        Self {
            id: network.id().0,
            name: network.name().to_string(),
            network_type: network.network_type().clone(),
            contact_email: network
                .contact_email()
                .as_ref()
                .map(|e| e.value().to_string()),
            phone_number: network
                .phone_number()
                .as_ref()
                .map(|p| p.value().to_string()),
            address: network.address().as_ref().map(|a| a.value().to_string()),
            owner_name: network.owner_name().to_string(),
            created_at: *network.created_at(),
            updated_at: *network.updated_at(),
            created_by: network.created_by().to_string(),
        }
    }
}
