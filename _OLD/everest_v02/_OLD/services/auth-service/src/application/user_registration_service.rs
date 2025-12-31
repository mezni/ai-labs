use crate::core::constants::*;
use crate::core::errors::AppError;
use crate::domain::{RegistrationStatus, UserRegistration};
use crate::infrastructure::keycloak_client::{CreateUserRequest, KeycloakClient};
use crate::infrastructure::repositories_pg::{
    log_keycloak_sync, RegistrationRepository, UserRepository,
};
use chrono::{Duration, Utc};
use nanoid::nanoid;
use sqlx::PgPool;

pub struct UserRegistrationService<R: RegistrationRepository, U: UserRepository> {
    registration_repo: R,
    user_repo: U,
    keycloak_client: KeycloakClient,
    pool: PgPool,
}

impl<R: RegistrationRepository, U: UserRepository> UserRegistrationService<R, U> {
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

    pub async fn register_user(
        &self,
        email: String,
        username: String,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
    ) -> Result<UserRegistration, AppError> {
        // Validate email format
        if !email.contains('@') {
            return Err(AppError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        // Check for existing user
        if self.user_repo.find_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        if self.user_repo.find_by_username(&username).await?.is_some() {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        // Create registration record
        let registration_id = format!("{}{}", REGISTRATION_ID_PREFIX, nanoid!(NANOID_LENGTH));
        let verification_token = nanoid!(32);
        let expires_at = Utc::now() + Duration::hours(REGISTRATION_EXPIRY_HOURS);

        let registration = UserRegistration {
            registration_id: registration_id.clone(),
            email: email.clone(),
            username: username.clone(),
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            phone: phone.clone(),
            verification_token,
            status: RegistrationStatus::Pending,
            keycloak_id: None,
            user_id: None,
            expires_at,
            verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Save registration
        self.registration_repo.create(&registration).await?;

        // Create user in Keycloak (disabled, pending verification)
        let keycloak_request = CreateUserRequest {
            username: username.clone(),
            email: email.clone(),
            enabled: false,
            email_verified: false,
            first_name,
            last_name,
            attributes: Some(serde_json::json!({
                "registration_id": registration_id,
            })),
            required_actions: Some(vec!["VERIFY_EMAIL".to_string()]),
        };

        match self.keycloak_client.create_user(keycloak_request).await {
            Ok(keycloak_id) => {
                // Update registration with Keycloak ID
                self.registration_repo
                    .update_keycloak_id(&registration_id, &keycloak_id)
                    .await?;

                // Log success
                log_keycloak_sync(
                    &self.pool,
                    None,
                    Some(&keycloak_id),
                    "create",
                    "success",
                    Some("User created in Keycloak"),
                    None,
                )
                .await?;

                // Send verification email
                if let Err(e) = self.keycloak_client.send_verify_email(&keycloak_id).await {
                    tracing::error!("Failed to send verification email: {}", e);
                }

                Ok(UserRegistration {
                    keycloak_id: Some(keycloak_id),
                    ..registration
                })
            }
            Err(e) => {
                tracing::error!("Failed to create user in Keycloak: {}", e);

                log_keycloak_sync(
                    &self.pool,
                    None,
                    None,
                    "create",
                    "failed",
                    None,
                    Some(&e.to_string()),
                )
                .await?;

                Err(e)
            }
        }
    }
}
