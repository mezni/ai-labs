use crate::AppState;
use crate::application::registration_dto::{
    RegisterRequest, RegisterResponse, ResendVerificationResponse,
};
use crate::core::{
    constants::{MAX_RESEND_COUNT, REGISTRATION_PREFIX, RESEND_COOLDOWN_MINUTES},
    errors::AppError,
    id_generator::{generate_id, generate_verification_token},
};
use crate::domain::{
    entities::UserRegistration,
    enums::{RegistrationStatus, Source},
    value_objects::{Email, Password, Username},
};
use crate::infrastructure::keycloak_client::KeycloakClient;
use actix_web::web;
use chrono::{TimeDelta, Utc};
use std::collections::HashMap;

pub struct RegistrationService;

impl RegistrationService {
    pub async fn register(
        state: web::Data<AppState>,
        request: RegisterRequest,
    ) -> Result<RegisterResponse, AppError> {
        // 1. Validation
        if request.password != request.confirm_password {
            return Err(AppError::ValidationError(
                "Passwords do not match".to_string(),
            ));
        }

        let email = Email::new(request.email)?;
        let username = Username::new(request.username)?;
        let password = Password::new(request.password)?;

        // 2. Duplicate Checks using AppState injected repositories
        if state
            .user_repo
            .find_by_email(email.value())
            .await?
            .is_some()
        {
            return Err(AppError::AlreadyExists("Email already registered".into()));
        }

        if state
            .user_repo
            .find_by_username(username.value())
            .await?
            .is_some()
        {
            return Err(AppError::AlreadyExists("Username already taken".into()));
        }

        // 3. Logic & Keycloak Prep
        let registration_id = generate_id(REGISTRATION_PREFIX);
        let verification_token = generate_verification_token();
        let expires_at = Utc::now()
            + TimeDelta::try_hours(state.config.verification_expiry_hours as i64)
                .unwrap_or_default();

        // Package attributes for Keycloak
        let mut attributes = HashMap::new();
        attributes.insert("email".to_string(), vec![email.value().to_string()]);
        attributes.insert("network_id".to_string(), vec!["-".to_string()]);
        attributes.insert("station_id".to_string(), vec!["-".to_string()]);
        attributes.insert("role".to_string(), vec!["user".to_string()]);
        attributes.insert(
            "verification_token".to_string(),
            vec![verification_token.clone()],
        );

        // 4. Keycloak Call
        let keycloak_id = state
            .keycloak_client
            .create_user(
                email.value(),
                username.value(),
                password.value(),
                Some(attributes),
            )
            .await?;

        // 5. Persistence via AppState repository
        let registration = UserRegistration {
            registration_id: registration_id.clone(),
            email: email.value().to_string(),
            username: username.value().to_string(),
            first_name: request.first_name,
            last_name: request.last_name,
            phone: request.phone,
            verification_token,
            status: RegistrationStatus::Pending,
            keycloak_id,
            user_id: None,
            resend_count: 0,
            expires_at,
            verified_at: None,
            created_at: Utc::now(),
            ip_address: None,
            user_agent: None,
            source: Source::Web,
        };

        state.reg_repo.create(&registration).await?;

        Ok(RegisterResponse {
            registration_id,
            email: email.value().to_string(),
            expires_at,
            message: "Registration created. Please verify your email.".to_string(),
        })
    }

    pub async fn resend_verification(
        state: web::Data<AppState>,
        email: String,
    ) -> Result<ResendVerificationResponse, AppError> {
        let email_obj = Email::new(email)?;

        // Using injected reg_repo from state
        let registration = state
            .reg_repo
            .find_by_email(email_obj.value())
            .await?
            .ok_or_else(|| AppError::NotFound("Registration not found".to_string()))?;

        if registration.status == RegistrationStatus::Verified {
            return Err(AppError::BadRequest("Email already verified".to_string()));
        }

        if registration.resend_count >= MAX_RESEND_COUNT {
            return Err(AppError::MaxResendAttemptsReached);
        }

        // Cooldown Logic
        if registration.resend_count > 0 {
            let cooldown_mins = RESEND_COOLDOWN_MINUTES * registration.resend_count as i64;
            let cooldown_end =
                registration.created_at + TimeDelta::try_minutes(cooldown_mins).unwrap_or_default();
            if Utc::now() < cooldown_end {
                return Err(AppError::ResendCooldownActive);
            }
        }

        let new_token = generate_verification_token();
        let new_expires_at = Utc::now()
            + TimeDelta::try_hours(state.config.verification_expiry_hours as i64)
                .unwrap_or_default();

        // Update via state repositories
        state
            .reg_repo
            .update_verification_token(&registration.registration_id, &new_token, new_expires_at)
            .await?;
        state
            .reg_repo
            .increment_resend_count(&registration.registration_id)
            .await?;

        let _ = state
            .keycloak_client
            .send_verification_email(&registration.keycloak_id)
            .await;

        Ok(ResendVerificationResponse {
            message: "Verification email resent".to_string(),
            expires_at: new_expires_at,
        })
    }
}
