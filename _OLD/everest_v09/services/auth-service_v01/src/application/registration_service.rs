use crate::core::constants::VERIFICATION_TOKEN_EXPIRY_HOURS;
use crate::core::errors::{AppError, AppResult};
use crate::domain::entities::{User, UserRegistration};
use crate::domain::enums::{RegistrationStatus, Source, UserRole};
use crate::domain::repositories::{RegistrationRepository, UserRepository};
use crate::domain::services::RegistrationService as RegistrationServiceTrait;
use crate::infrastructure::keycloak_client::KeycloakClient;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

pub struct RegistrationService {
    user_repo: Arc<dyn UserRepository>,
    registration_repo: Arc<dyn RegistrationRepository>,
    keycloak: Arc<dyn KeycloakClient>,
}

impl RegistrationService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        registration_repo: Arc<dyn RegistrationRepository>,
        keycloak: Arc<dyn KeycloakClient>,
    ) -> Self {
        Self {
            user_repo,
            registration_repo,
            keycloak,
        }
    }
}

#[async_trait]
impl RegistrationServiceTrait for RegistrationService {
    async fn register(
        &self,
        email: String,
        username: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        source: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<UserRegistration> {
        // Check if user already exists
        if self.user_repo.find_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".into()));
        }

        // Create user in Keycloak (disabled initially)
        let keycloak_id = self
            .keycloak
            .create_user(&email, &username, &password, None)
            .await
            .map_err(|e| AppError::Keycloak(e.to_string()))?;

        // Disable the user until email is verified
        self.keycloak
            .disable_user(&keycloak_id)
            .await
            .map_err(|e| AppError::Keycloak(e.to_string()))?;

        // Generate verification token
        let verification_token = nanoid::nanoid!(32);
        let expires_at = Utc::now() + chrono::Duration::hours(VERIFICATION_TOKEN_EXPIRY_HOURS);

        // Create registration record
        let registration = UserRegistration {
            registration_id: nanoid::nanoid!(32),
            email: email.clone(),
            username: username.clone(),
            first_name,
            last_name,
            phone,
            verification_token,
            status: RegistrationStatus::Pending,
            keycloak_id,
            user_id: None,
            resend_count: 0,
            expires_at,
            verified_at: None,
            created_at: Utc::now(),
            ip_address,
            user_agent,
            source: match source.as_str() {
                "mobile" => Source::Mobile,
                "internal" => Source::Internal,
                _ => Source::Web,
            },
        };

        let created = self.registration_repo.create(&registration).await?;

        // Send verification email
        self.keycloak
            .send_verification_email(&created.keycloak_id)
            .await
            .map_err(|e| AppError::Keycloak(e.to_string()))?;

        Ok(created)
    }

    async fn verify(&self, token: String) -> AppResult<User> {
        // Find registration by token
        let mut registration = self
            .registration_repo
            .find_by_token(&token)
            .await?
            .ok_or_else(|| AppError::NotFound("Invalid verification token".into()))?;

        // Check if already verified
        if registration.status == RegistrationStatus::Verified {
            return Err(AppError::BadRequest("Already verified".into()));
        }

        // Check if expired
        if Utc::now() > registration.expires_at {
            registration.status = RegistrationStatus::Expired;
            self.registration_repo.update(&registration).await?;
            return Err(AppError::BadRequest("Verification link expired".into()));
        }

        // Enable user in Keycloak
        self.keycloak
            .enable_user(&registration.keycloak_id)
            .await
            .map_err(|e| AppError::Keycloak(e.to_string()))?;

        // Create user in database
        let user = User {
            user_id: nanoid::nanoid!(32),
            keycloak_id: registration.keycloak_id.clone(),
            email: registration.email.clone(),
            username: registration.username.clone(),
            first_name: registration.first_name.clone(),
            last_name: registration.last_name.clone(),
            phone: registration.phone.clone(),
            photo: None,
            is_verified: true,
            role: UserRole::User,
            network_id: String::new(),
            station_id: String::new(),
            source: registration.source.clone(),
            is_active: true,
            deleted_at: None,
            last_login_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            updated_by: None,
        };

        let created_user = self.user_repo.create(&user).await?;

        // Update registration
        registration.status = RegistrationStatus::Verified;
        registration.user_id = Some(created_user.user_id.clone());
        registration.verified_at = Some(Utc::now());
        self.registration_repo.update(&registration).await?;

        Ok(created_user)
    }

    async fn resend_verification(&self, email: String) -> AppResult<()> {
        let mut registration = self
            .registration_repo
            .find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::NotFound("Registration not found".into()))?;

        if registration.status == RegistrationStatus::Verified {
            return Err(AppError::BadRequest("Already verified".into()));
        }

        if registration.resend_count >= 3 {
            return Err(AppError::BadRequest(
                "Maximum resend attempts exceeded".into(),
            ));
        }

        // Extend expiry
        registration.expires_at =
            Utc::now() + chrono::Duration::hours(VERIFICATION_TOKEN_EXPIRY_HOURS);
        registration.resend_count += 1;

        self.registration_repo.update(&registration).await?;

        // Resend email
        self.keycloak
            .send_verification_email(&registration.keycloak_id)
            .await
            .map_err(|e| AppError::Keycloak(e.to_string()))?;

        Ok(())
    }
}
