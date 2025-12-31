use crate::core::constants::*;
use crate::core::errors::AppError;
use crate::domain::{RegistrationStatus, User};
use crate::infrastructure::keycloak_client::KeycloakClient;
use crate::infrastructure::repositories_pg::{
    create_user_preferences, log_keycloak_sync, RegistrationRepository, UserRepository,
};
use nanoid::nanoid;
use sqlx::PgPool;

pub struct VerificationCallbackService<R: RegistrationRepository, U: UserRepository> {
    registration_repo: R,
    user_repo: U,
    keycloak_client: KeycloakClient,
    pool: PgPool,
}

impl<R: RegistrationRepository, U: UserRepository> VerificationCallbackService<R, U> {
    pub fn new(
        registration_repo: R,
        user_repo: U,
        keycloak_client: KeycloakClient,
        pool: PgPool,
    ) -> Self {
        Self {
            registration_repo,
            user_repo,
            keycloak_client,
            pool,
        }
    }

    pub async fn handle_verification(
        &self,
        keycloak_id: String,
        email: String,
    ) -> Result<(), AppError> {
        // Find registration by Keycloak ID
        let registration = self
            .registration_repo
            .find_by_keycloak_id(&keycloak_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Registration not found".to_string()))?;

        // Validate registration can be verified
        if !registration.can_verify() {
            return Err(AppError::ValidationError(
                "Registration cannot be verified".to_string(),
            ));
        }

        // Check if user already verified
        if registration.status == RegistrationStatus::Verified {
            return Err(AppError::Conflict("User already verified".to_string()));
        }

        // Create user ID
        let user_id = format!("{}{}", USER_ID_PREFIX, nanoid!(NANOID_LENGTH));

        // Create user record
        let user = User::new(
            user_id.clone(),
            keycloak_id.clone(),
            email,
            registration.username.clone(),
            registration.first_name.clone(),
            registration.last_name.clone(),
            registration.phone.clone(),
        );

        // Start transaction-like operations
        self.user_repo.create(&user).await?;

        // Update registration status
        self.registration_repo
            .mark_verified(&registration.registration_id, &user_id)
            .await?;

        // Create user preferences
        create_user_preferences(&self.pool, &user_id).await?;

        // Enable user in Keycloak
        match self.keycloak_client.enable_user(&keycloak_id).await {
            Ok(_) => {
                log_keycloak_sync(
                    &self.pool,
                    Some(&user_id),
                    Some(&keycloak_id),
                    "status_update",
                    "success",
                    Some("User enabled after verification"),
                    None,
                )
                .await?;
            }
            Err(e) => {
                tracing::error!("Failed to enable user in Keycloak: {}", e);

                log_keycloak_sync(
                    &self.pool,
                    Some(&user_id),
                    Some(&keycloak_id),
                    "status_update",
                    "failed",
                    None,
                    Some(&e.to_string()),
                )
                .await?;
            }
        }

        tracing::info!("User {} verified successfully", user_id);
        Ok(())
    }
}
