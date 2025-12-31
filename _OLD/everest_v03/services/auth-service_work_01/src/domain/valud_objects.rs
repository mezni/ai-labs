use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(raw: String) -> Result<Self, String> {
        if !raw.contains('@') {
            return Err("Invalid email".into());
        }
        Ok(Self(raw.to_lowercase()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationToken(String);

impl VerificationToken {
    pub fn new(value: String) -> Result<Self, String> {
        if value.len() < 16 {
            return Err("Verification token too short".into());
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct IpAddress(String);

impl IpAddress {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Expiry(DateTime<Utc>);

impl Expiry {
    pub fn new(expires_at: DateTime<Utc>) -> Result<Self, String> {
        Ok(Self(expires_at))
    }

    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        now > self.0
    }
}
