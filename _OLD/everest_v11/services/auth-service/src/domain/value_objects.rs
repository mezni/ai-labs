use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if email.contains('@') && email.len() >= 3 {
            Ok(Self(email))
        } else {
            Err("Invalid email format".to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, String> {
        if username.len() >= 3 && username.len() <= 50 {
            if username
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                Ok(Self(username))
            } else {
                Err("Username can only contain alphanumeric characters, underscores, and hyphens".to_string())
            }
        } else {
            Err("Username must be between 3 and 50 characters".to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self, String> {
        if password.len() >= 8 {
            Ok(Self(password))
        } else {
            Err("Password must be at least 8 characters".to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}