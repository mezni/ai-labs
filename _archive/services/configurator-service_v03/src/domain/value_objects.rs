use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Network ID value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct NetworkId(pub i32);

impl NetworkId {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_db(id: i32) -> Self {
        Self(id)
    }
}

impl Default for NetworkId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<i32> for NetworkId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

/// Network type enum
#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
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

macro_rules! string_value_object {
    ($name:ident, $min_len:expr, $err_msg:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: String) -> Result<Self, String> {
                if value.len() >= $min_len {
                    Ok(Self(value))
                } else {
                    Err($err_msg.to_string())
                }
            }

            pub fn value(&self) -> &str {
                &self.0
            }
        }
    };
}

string_value_object!(Email, 3, "Invalid email address");
string_value_object!(PhoneNumber, 6, "Invalid phone number");
string_value_object!(Address, 5, "Invalid address");