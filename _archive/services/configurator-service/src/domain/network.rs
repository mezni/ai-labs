use crate::domain::value_objects::{NetworkId, NetworkType, Email, PhoneNumber, Address};
use chrono::NaiveDateTime;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Network {
    pub id: NetworkId,
    pub name: String,
    pub network_type: NetworkType,
    pub contact_email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub address: Option<Address>,
    pub owner_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
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
}

#[async_trait]
pub trait NetworkRepository: Send + Sync {
    async fn create(&self, network: &Network) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: &NetworkId) -> anyhow::Result<Option<Network>>;
    async fn get_by_name(&self, name: &str) -> anyhow::Result<Option<Network>>;
    async fn get_all(&self) -> anyhow::Result<Vec<Network>>;
    async fn update(&self, network: &Network) -> anyhow::Result<()>;
    async fn delete(&self, id: &NetworkId) -> anyhow::Result<()>;
}
