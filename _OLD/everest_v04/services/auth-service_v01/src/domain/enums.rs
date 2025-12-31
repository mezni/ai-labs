use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RegistrationStatus {
    Pending,
    Verified,
    Expired,
    Cancelled,
}

impl fmt::Display for RegistrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegistrationStatus::Pending => write!(f, "pending"),
            RegistrationStatus::Verified => write!(f, "verified"),
            RegistrationStatus::Expired => write!(f, "expired"),
            RegistrationStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl From<String> for RegistrationStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => RegistrationStatus::Pending,
            "verified" => RegistrationStatus::Verified,
            "expired" => RegistrationStatus::Expired,
            "cancelled" => RegistrationStatus::Cancelled,
            _ => RegistrationStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
    Partner,
    Operator,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::Partner => write!(f, "partner"),
            UserRole::Operator => write!(f, "operator"),
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "user" => UserRole::User,
            "admin" => UserRole::Admin,
            "partner" => UserRole::Partner,
            "operator" => UserRole::Operator,
            _ => UserRole::User,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    Web,
    Internal,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Source::Web => write!(f, "web"),
            Source::Internal => write!(f, "internal"),
        }
    }
}

impl From<String> for Source {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "web" => Source::Web,
            "internal" => Source::Internal,
            _ => Source::Web,
        }
    }
}