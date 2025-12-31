use crate::AppState;
use crate::application::user_registration_dto::{
    RegisterUserRequest, RegisterUserResponse, VerifyRegistrationRequest,
    VerifyRegistrationResponse,
};
use crate::core::constants::REGISTRATION_EXPIRATION_HOURS;
use crate::core::errors::AppError;
use crate::core::id_generator::IdGenerator;
use crate::domain::user_registration::{RegistrationStatus, UserRegistration};
use chrono::{Duration, Utc};

pub struct UserRegistrationService;

impl UserRegistrationService {
    /// Matches the controller call: UserRegistrationService::execute
    pub async fn execute(
        state: &AppState,
        req: RegisterUserRequest,
    ) -> Result<RegisterUserResponse, AppError> {
        let exists = state
            .user_registration_repo
            .exists_by_email_or_username(&req.email, &req.username)
            .await?;

        if exists {
            return Err(AppError::Conflict("User already exists".into()));
        }

        let registration_id = IdGenerator::generate_registration_id();
        let verification_token = IdGenerator::generate_verification_token();

        let now = Utc::now();
        let expires_at = now + Duration::hours(REGISTRATION_EXPIRATION_HOURS);

        let user_reg = UserRegistration {
            registration_id: registration_id.clone(),
            email: req.email,
            username: req.username,
            first_name: req.first_name,
            last_name: req.last_name,
            phone: req.phone,
            verification_token,
            verification_code: None,
            status: RegistrationStatus::Pending,
            keycloak_id: None,
            user_id: None,
            expires_at,
            verified_at: None,
            created_at: now,
            updated_at: now,
            ip_address: None,
            user_agent: None,
        };

        state.user_registration_repo.save(&user_reg).await?;

        Ok(RegisterUserResponse {
            registration_id,
            status: user_reg.status.to_string(),
            expires_at: expires_at.to_rfc3339(),
        })
    }

    pub async fn verify(
        state: &AppState,
        req: VerifyRegistrationRequest,
    ) -> Result<VerifyRegistrationResponse, AppError> {
        // FIX: Ensure AppError::NotFound(String) exists in errors.rs
        let mut user_reg = state
            .user_registration_repo
            .find_by_id(&req.registration_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Invalid registration_id".into()))?;

        if !user_reg.can_verify() {
            return Err(AppError::BadRequest(
                "Cannot verify this registration".into(),
            ));
        }

        user_reg.status = RegistrationStatus::Verified;
        user_reg.verified_at = Some(Utc::now());
        user_reg.updated_at = Utc::now();

        state.user_registration_repo.save(&user_reg).await?;

        Ok(VerifyRegistrationResponse {
            registration_id: user_reg.registration_id,
            status: user_reg.status.to_string(),
            message: "Email verified successfully".into(),
        })
    }
}
