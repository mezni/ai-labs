use crate::application::register_dto::{RegisterRequest, RegisterResponse, ResendVerificationResponse};
use crate::core::{
    constants::{MAX_RESEND_COUNT, REGISTRATION_PREFIX, RESEND_COOLDOWN_MINUTES},
    errors::AppError,
    id_generator::{generate_id, generate_verification_token},
    state::AppState,
};
use crate::domain::{
    entities::UserRegistration,
    enums::{RegistrationStatus, Source},
    repositories::{RegistrationRepository, UserRepository},
    value_objects::{Email, Password, Username},
};
use crate::infrastructure::persistence::RegistrationRepositoryImpl;
use actix_web::web;
use chrono::{Duration, Utc};

pub struct RegistrationService;

impl RegistrationService {
    pub async fn register(
        state: web::Data<AppState>,
        request: RegisterRequest,
    ) -> Result<RegisterResponse, AppError> {
        // Validate input
        if request.password != request.confirm_password {
            return Err(AppError::ValidationError("Passwords do not match".to_string()));
        }

        let email = Email::new(request.email)?;
        let username = Username::new(request.username)?;
        let password = Password::new(request.password)?;

        // Check if email or username already exists in users table
        let user_repo = crate::infrastructure::persistence::UserRepositoryImpl::new(state.db_pool.clone());
        
        if let Some(_existing_user) = user_repo.find_by_email(email.value()).await? {
            return Err(AppError::AlreadyExists("Email already registered".to_string()));
        }

        if let Some(_existing_user) = user_repo.find_by_username(username.value()).await? {
            return Err(AppError::AlreadyExists("Username already taken".to_string()));
        }

        // Check for pending registration
        let reg_repo = RegistrationRepositoryImpl::new(state.db_pool.clone());
        if let Some(existing_reg) = reg_repo.find_by_email(email.value()).await? {
            if existing_reg.status == RegistrationStatus::Pending && existing_reg.expires_at > Utc::now() {
                return Err(AppError::AlreadyExists(
                    "A pending registration already exists for this email. Please check your email or wait for it to expire.".to_string()
                ));
            }
        }

        // Generate IDs and token
        let registration_id = generate_id(REGISTRATION_PREFIX);
        let verification_token = generate_verification_token();
        let expires_at = Utc::now() + Duration::hours(state.config.verification_expiry_hours);

        // Create user in Keycloak
        let keycloak_id = state
            .keycloak_client
            .create_user(
                username.value(),
                email.value(),
                password.value(),
                request.first_name.clone(),
                request.last_name.clone(),
                &verification_token,
            )
            .await?;

        // Send verification email (optional, doesn't fail registration)
        let _ = state
            .keycloak_client
            .send_verification_email(&keycloak_id)
            .await;

        // Create registration record
        let registration = UserRegistration {
            registration_id: registration_id.clone(),
            email: email.value().to_string(),
            username: username.value().to_string(),
            first_name: request.first_name,
            last_name: request.last_name,
            phone: request.phone,
            verification_token: verification_token.clone(),
            status: RegistrationStatus::Pending,
            keycloak_id,
            user_id: None,
            resend_count: 0,
            expires_at,
            verified_at: None,
            created_at: Utc::now(),
            ip_address: request.metadata.as_ref().and_then(|m| m.registration_ip.clone()),
            user_agent: request.metadata.as_ref().and_then(|m| m.user_agent.clone()),
            source: Source::from(request.source),
        };

        reg_repo.create(&registration).await?;

        tracing::info!("User registration created: {}", registration_id);

        Ok(RegisterResponse {
            registration_id,
            email: email.value().to_string(),
            expires_at,
            message: "Registration created. Check your email for verification.".to_string(),
        })
    }

    pub async fn resend_verification(
        state: web::Data<AppState>,
        email: String,
    ) -> Result<ResendVerificationResponse, AppError> {
        let email_obj = Email::new(email)?;

        let reg_repo = RegistrationRepositoryImpl::new(state.db_pool.clone());
        
        let registration = reg_repo
            .find_by_email(email_obj.value())
            .await?
            .ok_or_else(|| AppError::NotFound("No pending registration found for this email".to_string()))?;

        // Check if already verified
        if registration.status == RegistrationStatus::Verified {
            return Err(AppError::BadRequest("Email already verified".to_string()));
        }

        // Check if expired
        if registration.expires_at < Utc::now() {
            return Err(AppError::TokenExpired);
        }

        // Check resend count
        if registration.resend_count >= MAX_RESEND_COUNT {
            return Err(AppError::MaxResendAttemptsReached);
        }

        // Check cooldown (if previously sent)
        if registration.resend_count > 0 {
            let cooldown_end = registration.created_at + Duration::minutes(RESEND_COOLDOWN_MINUTES * registration.resend_count as i64);
            if Utc::now() < cooldown_end {
                return Err(AppError::ResendCooldownActive);
            }
        }

        // Generate new token and extend expiry
        let new_token = generate_verification_token();
        let new_expires_at = Utc::now() + Duration::hours(state.config.verification_expiry_hours);

        // Update registration
        reg_repo
            .update_verification_token(&registration.registration_id, &new_token, new_expires_at)
            .await?;
        
        reg_repo
            .increment_resend_count(&registration.registration_id)
            .await?;

        // Send verification email
        let _ = state
            .keycloak_client
            .send_verification_email(&registration.keycloak_id)
            .await;

        tracing::info!("Verification email resent for: {}", email_obj.value());

        Ok(ResendVerificationResponse {
            message: "Verification email sent successfully".to_string(),
            expires_at: new_expires_at,
        })
    }
}