use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::network::NetworkType;
use crate::domain::value_objects::{Email, PhoneNumber, Address};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateNetworkDto {
    pub name: String,
    pub network_type: NetworkType,
    pub contact_email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub address: Option<Address>,
    pub owner_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateNetworkDto {
    pub name: String,
    pub network_type: NetworkType,
    pub contact_email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub address: Option<Address>,
    pub owner_name: String,
}
