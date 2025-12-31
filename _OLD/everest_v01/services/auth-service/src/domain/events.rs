// src/domain/events.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Domain events that can be published for audit logging and event sourcing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DomainEvent {
    UserCreated(UserCreatedEvent),
    UserUpdated(UserUpdatedEvent),
    UserDeleted(UserDeletedEvent),
    UserLoggedIn(UserLoggedInEvent),
    UserLoggedOut(UserLoggedOutEvent),
    PasswordChanged(PasswordChangedEvent),
    PasswordResetRequested(PasswordResetRequestedEvent),
    PasswordReset(PasswordResetEvent),
    EmailVerified(EmailVerifiedEvent),
    RegistrationCreated(RegistrationCreatedEvent),
    RegistrationVerified(RegistrationVerifiedEvent),
    RoleChanged(RoleChangedEvent),
}

impl DomainEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            DomainEvent::UserCreated(_) => "user_created",
            DomainEvent::UserUpdated(_) => "user_updated",
            DomainEvent::UserDeleted(_) => "user_deleted",
            DomainEvent::UserLoggedIn(_) => "user_logged_in",
            DomainEvent::UserLoggedOut(_) => "user_logged_out",
            DomainEvent::PasswordChanged(_) => "password_changed",
            DomainEvent::PasswordResetRequested(_) => "password_reset_requested",
            DomainEvent::PasswordReset(_) => "password_reset",
            DomainEvent::EmailVerified(_) => "email_verified",
            DomainEvent::RegistrationCreated(_) => "registration_created",
            DomainEvent::RegistrationVerified(_) => "registration_verified",
            DomainEvent::RoleChanged(_) => "role_changed",
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            DomainEvent::UserCreated(e) => e.timestamp,
            DomainEvent::UserUpdated(e) => e.timestamp,
            DomainEvent::UserDeleted(e) => e.timestamp,
            DomainEvent::UserLoggedIn(e) => e.timestamp,
            DomainEvent::UserLoggedOut(e) => e.timestamp,
            DomainEvent::PasswordChanged(e) => e.timestamp,
            DomainEvent::PasswordResetRequested(e) => e.timestamp,
            DomainEvent::PasswordReset(e) => e.timestamp,
            DomainEvent::EmailVerified(e) => e.timestamp,
            DomainEvent::RegistrationCreated(e) => e.timestamp,
            DomainEvent::RegistrationVerified(e) => e.timestamp,
            DomainEvent::RoleChanged(e) => e.timestamp,
        }
    }

    pub fn user_id(&self) -> Option<&str> {
        match self {
            DomainEvent::UserCreated(e) => Some(&e.user_id),
            DomainEvent::UserUpdated(e) => Some(&e.user_id),
            DomainEvent::UserDeleted(e) => Some(&e.user_id),
            DomainEvent::UserLoggedIn(e) => Some(&e.user_id),
            DomainEvent::UserLoggedOut(e) => Some(&e.user_id),
            DomainEvent::PasswordChanged(e) => Some(&e.user_id),
            DomainEvent::PasswordResetRequested(e) => Some(&e.user_id),
            DomainEvent::PasswordReset(e) => Some(&e.user_id),
            DomainEvent::EmailVerified(e) => Some(&e.user_id),
            DomainEvent::RegistrationCreated(_) => None,
            DomainEvent::RegistrationVerified(e) => Some(&e.user_id),
            DomainEvent::RoleChanged(e) => Some(&e.user_id),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: String,
    pub keycloak_id: String,
    pub email: String,
    pub username: String,
    pub role: String,
    pub source: String,
    pub created_by: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: String,
    pub updated_by: String,
    pub changes: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeletedEvent {
    pub user_id: String,
    pub deleted_by: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedInEvent {
    pub user_id: String,
    pub keycloak_id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedOutEvent {
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChangedEvent {
    pub user_id: String,
    pub changed_by: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequestedEvent {
    pub user_id: String,
    pub email: String,
    pub ip_address: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetEvent {
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerifiedEvent {
    pub user_id: String,
    pub email: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationCreatedEvent {
    pub registration_id: String,
    pub email: String,
    pub username: String,
    pub ip_address: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationVerifiedEvent {
    pub registration_id: String,
    pub user_id: String,
    pub keycloak_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleChangedEvent {
    pub user_id: String,
    pub old_role: String,
    pub new_role: String,
    pub changed_by: String,
    pub timestamp: DateTime<Utc>,
}

// Event builder helpers
impl UserCreatedEvent {
    pub fn new(
        user_id: String,
        keycloak_id: String,
        email: String,
        username: String,
        role: String,
        source: String,
        created_by: Option<String>,
    ) -> Self {
        Self {
            user_id,
            keycloak_id,
            email,
            username,
            role,
            source,
            created_by,
            timestamp: Utc::now(),
        }
    }
}

impl UserLoggedInEvent {
    pub fn new(
        user_id: String,
        keycloak_id: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        Self {
            user_id,
            keycloak_id,
            ip_address,
            user_agent,
            timestamp: Utc::now(),
        }
    }
}

impl PasswordChangedEvent {
    pub fn new(user_id: String, changed_by: String) -> Self {
        Self {
            user_id,
            changed_by,
            timestamp: Utc::now(),
        }
    }
}

impl RoleChangedEvent {
    pub fn new(user_id: String, old_role: String, new_role: String, changed_by: String) -> Self {
        Self {
            user_id,
            old_role,
            new_role,
            changed_by,
            timestamp: Utc::now(),
        }
    }
}
