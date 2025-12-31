use crate::domain::value_objects::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum RegistrationStatus {
    Pending,
    Verified,
    Expired,
    Cancelled,
    Failed,
    EmailFailed,
    KeycloakFailed,
}

#[derive(Debug, Clone)]
pub enum VerificationMethod {
    Email,
    Sms,
    Manual,
}

#[derive(Debug, Clone)]
pub enum RegistrationSource {
    Web,
    Mobile,
    Api,
    Admin,
    Invitation,
}

#[derive(Debug)]
pub struct UserRegistration {
    id: Uuid,
    email: Email,
    verification_token: VerificationToken,
    verification_method: VerificationMethod,
    status: RegistrationStatus,
    expires_at: Expiry,
    verified_at: Option<DateTime<Utc>>,
    registration_ip: Option<IpAddress>,
}

impl UserRegistration {
    pub fn start(
        email: Email,
        token: VerificationToken,
        expires_at: Expiry,
        method: VerificationMethod,
        ip: Option<IpAddress>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            verification_token: token,
            verification_method: method,
            status: RegistrationStatus::Pending,
            expires_at,
            verified_at: None,
            registration_ip: ip,
        }
    }

    pub fn verify(&mut self, now: DateTime<Utc>) -> Result<(), String> {
        if self.expires_at.is_expired(now) {
            self.status = RegistrationStatus::Expired;
            return Err("Registration expired".into());
        }

        self.status = RegistrationStatus::Verified;
        self.verified_at = Some(now);
        Ok(())
    }

    pub fn mark_keycloak_failed(&mut self) {
        self.status = RegistrationStatus::KeycloakFailed;
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn status(&self) -> &RegistrationStatus {
        &self.status
    }
}
