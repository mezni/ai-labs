use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")] // Changed to varchar
pub enum UserRole {
    User,
    Admin,
    Partner,
    Operator,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")] // Changed to varchar
pub enum RegistrationStatus {
    Created,
    Pending,
    Verified,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)] // Added PartialEq for consistency
#[sqlx(type_name = "varchar", rename_all = "lowercase")] // Changed to varchar
pub enum Source {
    Web,
    Mobile,
    Internal,
}
