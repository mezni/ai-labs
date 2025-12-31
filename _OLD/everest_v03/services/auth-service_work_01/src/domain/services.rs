use crate::domain::{
    registration::UserRegistration,
    user::User,
    value_objects::{Email, VerificationToken},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Domain service responsible for starting user registrations
///
/// Business responsibility:
/// - Ensure registration can begin
/// - Enforce domain-level invariants
pub trait RegistrationService {
    fn start_registration(
        &self,
        email: Email,
        expires_at: DateTime<Utc>,
    ) -> Result<UserRegistration, String>;
}

/// Domain service responsible for verification logic
///
/// Business responsibility:
/// - Validate token
/// - Transition registration state
pub trait VerificationService {
    fn verify_registration(
        &self,
        token: VerificationToken,
        verified_at: DateTime<Utc>,
    ) -> Result<UserRegistration, String>;
}

/// Domain service responsible for user creation
///
/// Business responsibility:
/// - Create a User aggregate from a verified registration
/// - Enforce lifecycle rules
pub trait UserActivationService {
    fn activate_user(
        &self,
        registration_id: Uuid,
        keycloak_id: String,
    ) -> Result<User, String>;
}

/// Domain service for Keycloak synchronization state handling
///
/// Business responsibility:
/// - Decide how Keycloak sync impacts domain state
pub trait KeycloakSyncService {
    fn mark_sync_success(
        &self,
        registration_id: Uuid,
        synced_at: DateTime<Utc>,
    ) -> Result<(), String>;

    fn mark_sync_failure(
        &self,
        registration_id: Uuid,
        error: String,
    ) -> Result<(), String>;
}
