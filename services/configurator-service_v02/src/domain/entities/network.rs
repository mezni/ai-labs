// src/domain/entities/network.rs
use crate::domain::value_objects::{NetworkId, NetworkType, Email, PhoneNumber, Address};
use chrono::NaiveDateTime;

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
    pub fn new_from_dto(dto: &crate::application::dtos::network_dtos::CreateNetworkDto) -> Self {
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

    pub fn update_from_dto(&mut self, dto: &crate::application::dtos::network_dtos::UpdateNetworkDto) {
        self.name = dto.name.clone();
        self.network_type = dto.network_type.clone().into();
        self.contact_email = dto.contact_email.clone();
        self.phone_number = dto.phone_number.clone();
        self.address = dto.address.clone();
        self.owner_name = dto.owner_name.clone();
        self.updated_at = chrono::Utc::now().naive_utc();
    }

    // Add getters for private fields if needed for domain services
    pub fn id(&self) -> &NetworkId { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn network_type(&self) -> &NetworkType { &self.network_type }
    pub fn contact_email(&self) -> &Option<Email> { &self.contact_email }
    pub fn phone_number(&self) -> &Option<PhoneNumber> { &self.phone_number }
    pub fn address(&self) -> &Option<Address> { &self.address }
}
