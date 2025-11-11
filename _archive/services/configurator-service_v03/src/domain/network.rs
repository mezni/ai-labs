use crate::domain::value_objects::{Address, Email, NetworkId, NetworkType, PhoneNumber};
use chrono::NaiveDateTime;
use async_trait::async_trait;

/// Represents a network in the system.
#[derive(Debug, Clone)]
pub struct Network {
    id: NetworkId,
    name: String,
    network_type: NetworkType,
    contact_email: Option<Email>,
    phone_number: Option<PhoneNumber>,
    address: Option<Address>,
    owner_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    created_by: String,
}

impl Network {
    /// Creates a new `Network` instance from a `CreateNetworkDto`.
    pub fn new_from_dto(dto: &crate::application::dtos::network_dtos::CreateNetworkDto) -> Self {
        // Validation logic
        assert!(!dto.name.is_empty(), "Name is required");
        assert!(!dto.owner_name.is_empty(), "Owner name is required");

        let now = chrono::Utc::now().naive_utc();
        Self {
            id: NetworkId::new(),
            name: dto.name.clone(),
            network_type: dto.network_type.clone().into(),
            contact_email: dto.contact_email.clone(),
            phone_number: dto.phone_number.clone(),
            address: dto.address.clone(),
            owner_name: dto.owner_name.clone(),
            created_at: now,
            updated_at: now,
            created_by: "system".into(),
        }
    }

    /// Updates an existing `Network` instance from an `UpdateNetworkDto`.
    pub fn update_from_dto(
        &mut self,
        dto: &crate::application::dtos::network_dtos::UpdateNetworkDto,
    ) {
        // Validation logic
        assert!(!dto.name.is_empty(), "Name is required");
        assert!(!dto.owner_name.is_empty(), "Owner name is required");

        self.name = dto.name.clone();
        self.network_type = dto.network_type.clone().into();
        self.contact_email = dto.contact_email.clone();
        self.phone_number = dto.phone_number.clone();
        self.address = dto.address.clone();
        self.owner_name = dto.owner_name.clone();
        self.updated_at = chrono::Utc::now().naive_utc();
    }

    // Getters
    pub fn id(&self) -> &NetworkId {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn network_type(&self) -> &NetworkType {
        &self.network_type
    }
    pub fn contact_email(&self) -> &Option<Email> {
        &self.contact_email
    }
    pub fn phone_number(&self) -> &Option<PhoneNumber> {
        &self.phone_number
    }
    pub fn address(&self) -> &Option<Address> {
        &self.address
    }
}

#[async_trait]
pub trait NetworkRepository: Send + Sync {
    /// Creates a new network in the repository.
    async fn create(&self, network: &Network) -> anyhow::Result<()>;
    /// Retrieves a network by its ID.
    async fn get_by_id(&self, id: &NetworkId) -> anyhow::Result<Option<Network>>;
    /// Retrieves a network by its name.
    async fn get_by_name(&self, name: &str) -> anyhow::Result<Option<Network>>;
    /// Retrieves all networks in the repository.
    async fn get_all(&self) -> anyhow::Result<Vec<Network>>;
    /// Updates an existing network in the repository.
    async fn update(&self, network: &Network) -> anyhow::Result<()>;
    /// Deletes a network from the repository.
    async fn delete(&self, id: &NetworkId) -> anyhow::Result<()>;
}