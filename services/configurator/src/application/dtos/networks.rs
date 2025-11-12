use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// DTO for creating a network - REMOVE created_by since it comes from auth
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NetworkCreate {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
}

// DTO for updating a network - REMOVE updated_by since it comes from auth
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NetworkUpdate {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
}