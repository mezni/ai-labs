use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Inactive,
    PendingVerification,
    VerificationExpired,
    Suspended,
}

impl UserStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::PendingVerification => "pending_verification",
            Self::VerificationExpired => "verification_expired",
            Self::Suspended => "suspended",
        }
    }
}

impl From<String> for UserStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "active" => Self::Active,
            "inactive" => Self::Inactive,
            "pending_verification" => Self::PendingVerification,
            "verification_expired" => Self::VerificationExpired,
            "suspended" => Self::Suspended,
            _ => Self::Inactive,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Admin => "admin",
            Self::User => "user",
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "admin" => Self::Admin,
            _ => Self::User,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Expired,
    Cancelled,
}

impl InvitationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
        }
    }
}

impl From<String> for InvitationStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pending" => Self::Pending,
            "accepted" => Self::Accepted,
            "expired" => Self::Expired,
            "cancelled" => Self::Cancelled,
            _ => Self::Pending,
        }
    }
}