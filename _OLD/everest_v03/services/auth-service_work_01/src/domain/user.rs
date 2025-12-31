use crate::domain::value_objects::{Email, IpAddress};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum UserStatus {
    Pending,
    Active,
    Inactive,
    Locked,
    Suspended,
    Deleted,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Admin,
    Partner,
    Operator,
}

#[derive(Debug)]
pub struct User {
    id: Uuid,
    keycloak_id: String,
    email: Email,
    status: UserStatus,
    role: UserRole,
    email_verified: bool,
    created_at: DateTime<Utc>,
    last_login_at: Option<DateTime<Utc>>,
    registration_ip: Option<IpAddress>,
}

impl User {
    pub fn activate(
        registration_id: Uuid,
        keycloak_id: String,
        email: Email,
        ip: Option<IpAddress>,
    ) -> Self {
        Self {
            id: registration_id,
            keycloak_id,
            email,
            status: UserStatus::Active,
            role: UserRole::User,
            email_verified: true,
            created_at: Utc::now(),
            last_login_at: None,
            registration_ip: ip,
        }
    }

    pub fn lock(&mut self) {
        self.status = UserStatus::Locked;
    }

    pub fn record_login(&mut self, at: DateTime<Utc>) {
        self.last_login_at = Some(at);
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }
}
