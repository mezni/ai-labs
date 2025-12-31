use crate::domain::{
    registration::UserRegistration,
    repositories::UserRegistrationRepository,
    services::{RegistrationService, VerificationService},
    value_objects::{Email, VerificationToken},
};
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub struct RegistrationAppService {
    registration_repo: Arc<dyn UserRegistrationRepository>,
}

impl RegistrationAppService {
    pub fn new(registration_repo: Arc<dyn UserRegistrationRepository>) -> Self {
        Self { registration_repo }
    }
}

impl RegistrationService for RegistrationAppService {
    fn start_registration(
        &self,
        email: Email,
        expires_at: DateTime<Utc>,
    ) -> Result<UserRegistration, String> {
        if self
            .registration_repo
            .exists_pending_by_email(email.value())
        {
            return Err("Pending registration already exists".into());
        }

        let token = VerificationToken::new(uuid::Uuid::new_v4().to_string())?;
        let registration =
            UserRegistration::start(email, token, expires_at.into(), None);

        self.registration_repo.save(&registration);
        Ok(registration)
    }
}

impl VerificationService for RegistrationAppService {
    fn verify_registration(
        &self,
        token: VerificationToken,
        verified_at: DateTime<Utc>,
    ) -> Result<UserRegistration, String> {
        let mut registration = self
            .registration_repo
            .find_by_token(token.value())
            .ok_or("Invalid verification token")?;

        registration.verify(verified_at)?;
        self.registration_repo.save(&registration);

        Ok(registration)
    }
}
