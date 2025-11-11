use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct NetworkId(pub i32);

impl NetworkId { pub fn new() -> Self { Self(0) } }

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub enum NetworkType { Individual, Company }

macro_rules! string_value_object {
    ($name:ident, $min_len:expr, $err_msg:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
        pub struct $name(String);
        impl $name {
            pub fn new(value: String) -> Result<Self, String> {
                if value.len() >= $min_len { Ok(Self(value)) } else { Err($err_msg.to_string()) }
            }
            pub fn value(&self) -> &str { &self.0 }
        }
    };
}

string_value_object!(Email, 3, "Invalid email");
string_value_object!(PhoneNumber, 6, "Invalid phone number");
string_value_object!(Address, 5, "Invalid address");
