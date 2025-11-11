use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkId(pub i32);

impl NetworkId {
    pub fn new() -> Self {
        Self(rand::random::<i32>()) 
    }
}

impl From<i32> for NetworkId {
    fn from(id: i32) -> Self { Self(id) }
}


#[derive(Debug, Clone, ToSchema)]
pub enum NetworkType {
    Individual,
    Company,
}
impl NetworkType {
    pub fn from_str(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "individual" => Some(Self::Individual),
            "company" => Some(Self::Company),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Individual => "individual",
            Self::Company => "company",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if email.contains('@') {
            Ok(Self(email))
        } else {
            Err("Invalid email address".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone_number: String) -> Result<Self, String> {
        if phone_number.len() >= 6 {
            Ok(Self(phone_number))
        } else {
            Err("Invalid phone number".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    pub fn new(address: String) -> Result<Self, String> {
        if address.len() >= 5 {
            Ok(Self(address))
        } else {
            Err("Invalid address".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
