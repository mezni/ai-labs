use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct NetworkId(pub i32);

impl From<i32> for NetworkId {
    fn from(value: i32) -> Self {
        NetworkId(value)
    }
}

impl From<NetworkId> for i32 {
    fn from(value: NetworkId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct ContactInfo {
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "contact@example.com")]
    pub email: Option<String>,

    #[validate(length(max = 50, message = "Phone number too long"))]
    #[schema(example = "+1234567890")]
    pub phone: Option<String>,

    #[validate(length(max = 1000, message = "Address too long"))]
    #[schema(example = "123 Main St, City, Country")]
    pub address: Option<String>,
}

impl ContactInfo {
    pub fn new(
        email: Option<String>,
        phone: Option<String>,
        address: Option<String>,
    ) -> Result<Self, DomainError> {
        let contact = Self {
            email,
            phone,
            address,
        };

        if let Some(phone) = &contact.phone {
            if phone.len() > 50 {
                return Err(DomainError::PhoneTooLong(phone.len(), 50));
            }
        }

        if let Some(address) = &contact.address {
            if address.len() > 1000 {
                return Err(DomainError::PhoneTooLong(address.len(), 1000));
            }
        }

        Ok(contact)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditInfo {
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AuditInfo {
    pub fn new(created_by: String) -> Result<Self, DomainError> {
        if created_by.trim().is_empty() {
            return Err(DomainError::EmptyCreatedBy);
        }
        let now = chrono::Utc::now();
        Ok(Self {
            created_by,
            updated_by: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn update_audit(&mut self, updated_by: String) {
        self.updated_by = Some(updated_by);
        self.updated_at = chrono::Utc::now();
    }
}
