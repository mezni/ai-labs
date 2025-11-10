use crate::domain::errors::DomainError;
use crate::domain::value_objects::{AuditInfo, ContactInfo, NetworkId};
use serde::{Deserialize, Serialize};
use shared::constants;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum NetworkType {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "company")]
    Company,
}

impl NetworkType {
    pub fn as_str(&self) -> &str {
        match self {
            NetworkType::Individual => constants::NETWORK_TYPE_INDIVIDUAL,
            NetworkType::Company => constants::NETWORK_TYPE_COMPANY,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            constants::NETWORK_TYPE_INDIVIDUAL => Ok(NetworkType::Individual),
            constants::NETWORK_TYPE_COMPANY => Ok(NetworkType::Company),
            _ => Err(DomainError::InvalidNetworkType(s.to_string())),
        }
    }
}

impl std::fmt::Display for NetworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Network {
    pub id: NetworkId,
    pub name: String,
    pub network_type: NetworkType,
    pub contact_info: ContactInfo,
    pub audit_info: AuditInfo,
}

impl Network {
    pub fn new(
        name: String,
        network_type: NetworkType,
        contact_info: ContactInfo,
        created_by: String,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::EmptyName);
        }
        if name.len() > constants::MAX_NAME_LENGTH {
            return Err(DomainError::NameTooLong(name.len(), constants::MAX_NAME_LENGTH));
        }

        let audit_info = AuditInfo::new(created_by)?;

        Ok(Self {
            id: NetworkId(0),
            name: name.trim().to_string(),
            network_type,
            contact_info,
            audit_info,
        })
    }

    pub fn update(
        &mut self,
        name: Option<String>,
        network_type: Option<NetworkType>,
        contact_info: Option<ContactInfo>,
        updated_by: String,
    ) -> Result<(), DomainError> {
        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(DomainError::EmptyName);
            }
            if new_name.len() > constants::MAX_NAME_LENGTH {
                return Err(DomainError::NameTooLong(new_name.len(), constants::MAX_NAME_LENGTH));
            }
            self.name = new_name.trim().to_string();
        }

        if let Some(new_type) = network_type {
            self.network_type = new_type;
        }

        if let Some(new_contact_info) = contact_info {
            self.contact_info = new_contact_info;
        }

        self.audit_info.update_audit(updated_by);
        Ok(())
    }

    pub fn change_contact_email(&mut self, email: Option<String>, updated_by: String) {
        self.contact_info.email = email;
        self.audit_info.update_audit(updated_by);
    }

    pub fn change_contact_phone(&mut self, phone: Option<String>, updated_by: String) {
        self.contact_info.phone = phone;
        self.audit_info.update_audit(updated_by);
    }

    pub fn change_address(&mut self, address: Option<String>, updated_by: String) {
        self.contact_info.address = address;
        self.audit_info.update_audit(updated_by);
    }
}

// Builder pattern for Network
pub struct NetworkBuilder {
    name: Option<String>,
    network_type: Option<NetworkType>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    created_by: Option<String>,
}

impl NetworkBuilder {
    pub fn new() -> Self {
        Self { name: None, network_type: None, email: None, phone: None, address: None, created_by: None }
    }

    pub fn name(mut self, name: String) -> Self { self.name = Some(name); self }
    pub fn network_type(mut self, network_type: NetworkType) -> Self { self.network_type = Some(network_type); self }
    pub fn email(mut self, email: String) -> Self { self.email = Some(email); self }
    pub fn phone(mut self, phone: String) -> Self { self.phone = Some(phone); self }
    pub fn address(mut self, address: String) -> Self { self.address = Some(address); self }
    pub fn created_by(mut self, created_by: String) -> Self { self.created_by = Some(created_by); self }

    pub fn build(self) -> Result<Network, DomainError> {
        let name = self.name.ok_or(DomainError::EmptyName)?;
        let network_type = self.network_type.ok_or(DomainError::InvalidNetworkType("Not set".into()))?;
        let created_by = self.created_by.ok_or(DomainError::EmptyCreatedBy)?;
        let contact_info = ContactInfo::new(self.email, self.phone, self.address)?;
        Network::new(name, network_type, contact_info, created_by)
    }
}
