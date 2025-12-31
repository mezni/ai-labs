use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
    Partner,
    Operator,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum RegistrationStatus {
    Created,
    Pending,
    Verified,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum Source {
    Web,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Expired,
    Cancelled,
}
