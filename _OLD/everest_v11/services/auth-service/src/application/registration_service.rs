use std::sync::Arc;

use crate::application::dtos::registration::{RegisterRequest, ResendVerificationRequest, VerifyRequest};
use crate::core::{constants::*, errors::{AppError, AppResult}};
use crate::domain::repositories::RegistrationRepository;
use crate::infrastructure::keycloak_client::KeycloakClient;

pub struct RegistrationService {
    registration_repo: Arc<dyn RegistrationRepository>,
    keycloak_client: Arc<dyn KeycloakClient>,
}

impl RegistrationService {
    pub fn new(
        registration_repo: Arc<dyn RegistrationRepository>,
        keycloak_client: Arc<dyn KeycloakClient>,
    ) -> Self {
        Self {
            registration_repo,
            keycloak_client,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> AppResult<()> {
        tracing::info!("Registering user: {}", req.email);

        // Check if user already exists in registration table
        if let Ok(_) = self.registration_repo.find_by_email(&req.email).await {
            return Err(AppError::Conflict(
                "A registration with this email already exists".into(),
            ));
        }

        // Create user in Keycloak (disabled until email is verified)
        let keycloak_id = self
            .keycloak_client
            .create_user(&req.email, &req.username, &req.password, None)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        // Disable the user initially
        self.keycloak_client
            .disable_user(&keycloak_id)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        // Create registration record
        let verification_expires_at =
            chrono::Utc::now() + chrono::Duration::hours(VERIFICATION_EXPIRY_HOURS);

        self.registration_repo
            .create(
                &req.email,
                &req.username,
                &keycloak_id,
                verification_expires_at,
            )
            .await?;

        // Send verification email via Keycloak
        self.keycloak_client
            .send_verification_email(&keycloak_id)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        tracing::info!("Registration successful for: {}", req.email);
        Ok(())
    }

    pub async fn verify(&self, req: VerifyRequest) -> AppResult<()> {
        tracing::info!("Verifying email: {}", req.email);

        let registration = self.registration_repo.find_by_email(&req.email).await?;

        // Check if already verified
        if registration.status == STATUS_ACTIVE {
            return Err(AppError::AlreadyVerified);
        }

        // Check if verification expired
        if let Some(expires_at) = registration.verification_expires_at {
            if chrono::Utc::now() > expires_at {
                // Mark as expired and delete from Keycloak
                self.registration_repo
                    .update_status(&registration.id, STATUS_VERIFICATION_EXPIRED)
                    .await?;

                if let Some(keycloak_id) = &registration.keycloak_id {
                    // In production, you'd actually delete the Keycloak user here
                    tracing::warn!("Should delete Keycloak user: {}", keycloak_id);
                }

                return Err(AppError::VerificationExpired);
            }
        }

        // Enable user in Keycloak
        if let Some(keycloak_id) = &registration.keycloak_id {
            self.keycloak_client
                .enable_user(keycloak_id)
                .await
                .map_err(|e| AppError::KeycloakError(e.to_string()))?;
        }

        // Update registration status
        self.registration_repo
            .update_status(&registration.id, STATUS_ACTIVE)
            .await?;

        tracing::info!("Email verified successfully: {}", req.email);
        Ok(())
    }

    pub async fn resend_verification(&self, req: ResendVerificationRequest) -> AppResult<()> {
        tracing::info!("Resending verification for: {}", req.email);

        let registration = self.registration_repo.find_by_email(&req.email).await?;

        // Check if already verified
        if registration.status == STATUS_ACTIVE {
            return Err(AppError::AlreadyVerified);
        }

        // Check if verification expired
        if let Some(expires_at) = registration.verification_expires_at {
            if chrono::Utc::now() > expires_at {
                return Err(AppError::VerificationExpired);
            }
        }

        // Check resend attempts
        if registration.resend_count >= MAX_RESEND_ATTEMPTS {
            return Err(AppError::TooManyResendAttempts);
        }

        // Check cooldown period
        if let Some(last_resend) = registration.last_resend_at {
            let cooldown = chrono::Duration::minutes(RESEND_COOLDOWN_MINUTES);
            if chrono::Utc::now() < last_resend + cooldown {
                return Err(AppError::BadRequest(
                    "Please wait before requesting another verification email".into(),
                ));
            }
        }

        // Resend verification email
        if let Some(keycloak_id) = &registration.keycloak_id {
            self.keycloak_client
                .send_verification_email(keycloak_id)
                .await
                .map_err(|e| AppError::KeycloakError(e.to_string()))?;
        }

        // Update resend count and timestamp
        self.registration_repo
            .increment_resend_count(&registration.id)
            .await?;

        tracing::info!("Verification email resent: {}", req.email);
        Ok(())
    }
}