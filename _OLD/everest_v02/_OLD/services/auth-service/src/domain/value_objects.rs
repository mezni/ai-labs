use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, String> {
        if username.len() < 3 || username.len() > 100 {
            return Err("Username must be between 3 and 100 characters".to_string());
        }
        Ok(Self(username))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
