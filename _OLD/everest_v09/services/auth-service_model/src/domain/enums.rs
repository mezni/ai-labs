use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "text")]
pub enum UserRole {
    #[sqlx(rename = "user")]
    User,
    #[sqlx(rename = "admin")]
    Admin,
    #[sqlx(rename = "partner")]
    Partner,
    #[sqlx(rename = "operator")]
    Operator,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "text")]
pub enum RegistrationStatus {
    #[sqlx(rename = "pending")]
    Pending,
    #[sqlx(rename = "verified")]
    Verified,
    #[sqlx(rename = "expired")]
    Expired,
    #[sqlx(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "text")]
pub enum Source {
    #[sqlx(rename = "web")]
    Web,
    #[sqlx(rename = "mobile")]
    Mobile,
    #[sqlx(rename = "internal")]
    Internal,
}
