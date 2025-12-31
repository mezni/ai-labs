use crate::AppState;
use crate::application::authentication_dto::*;
use crate::core::{
    constants::{TOKEN_PREFIX, USER_PREFIX},
    errors::AppError,
    id_generator::generate_id,
};
use crate::domain::{
    entities::{RefreshToken, User},
    enums::{RegistrationStatus, UserRole},
};
use actix_web::web;
use chrono::{Duration, Utc};
// Ensure the trait is in scope to use its methods
use crate::infrastructure::keycloak_client::KeycloakClient;

pub struct AuthenticationService;

impl AuthenticationService {
    pub async fn verify(
        state: web::Data<AppState>,
        request: VerifyRequest,
    ) -> Result<VerifyResponse, AppError> {
        // 1. Use state.reg_repo instead of local Impl
        let registration = state
            .reg_repo
            .find_by_token(&request.token)
            .await?
            .ok_or_else(|| AppError::NotFound("Invalid token".to_string()))?;

        if registration.status == RegistrationStatus::Verified {
            return Err(AppError::BadRequest("Already verified".to_string()));
        }

        if registration.expires_at < Utc::now() {
            state
                .reg_repo
                .update_status(&registration.registration_id, "expired")
                .await?;
            return Err(AppError::TokenExpired);
        }

        // 2. Enable in Keycloak
        state
            .keycloak_client
            .enable_user(&registration.keycloak_id)
            .await?;

        // 3. Create local User
        let user_id = generate_id(USER_PREFIX);
        let user = User {
            user_id: user_id.clone(),
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
            last_login_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            updated_by: None,
        };

        // Use state.user_repo and state.reg_repo
        state.user_repo.create(&user).await?;
        state
            .reg_repo
            .update_status(&registration.registration_id, "verified")
            .await?;
        state
            .reg_repo
            .update_user_id(&registration.registration_id, &user_id)
            .await?;

        // 4. Authenticate
        let token_data = state
            .keycloak_client
            .authenticate(&registration.username, "temporary_or_stored_password")
            .await?;

        let refresh_token_entity = RefreshToken {
            token_id: generate_id(TOKEN_PREFIX),
            user_id: user_id.clone(),
            refresh_token: token_data.refresh_token.clone(),
            expires_at: Utc::now() + Duration::days(state.config.refresh_token_expiry_days as i64),
            created_at: Utc::now(),
            revoked_at: None,
            ip_address: request
                .metadata
                .as_ref()
                .and_then(|m| m.verification_ip.clone()),
            user_agent: request.metadata.as_ref().and_then(|m| m.user_agent.clone()),
        };

        // Use state.token_repo
        state.token_repo.create(&refresh_token_entity).await?;

        Ok(VerifyResponse {
            user_id,
            email: user.email,
            message: "Verified successfully".to_string(),
            access_token: token_data.access_token,
            refresh_token: token_data.refresh_token,
            expires_in: token_data.expires_in,
        })
    }

    pub async fn login(
        state: web::Data<AppState>,
        request: LoginRequest,
    ) -> Result<LoginResponse, AppError> {
        let token_data = state
            .keycloak_client
            .authenticate(&request.username, &request.password)
            .await?;

        // Use state.user_repo
        let user = state
            .user_repo
            .find_by_username(&request.username)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        state.user_repo.update_last_login(&user.user_id).await?;

        let refresh_entity = RefreshToken {
            token_id: generate_id(TOKEN_PREFIX),
            user_id: user.user_id.clone(),
            refresh_token: token_data.refresh_token.clone(),
            expires_at: Utc::now() + Duration::days(state.config.refresh_token_expiry_days as i64),
            created_at: Utc::now(),
            revoked_at: None,
            ip_address: request.metadata.as_ref().and_then(|m| m.login_ip.clone()),
            user_agent: request.metadata.as_ref().and_then(|m| m.user_agent.clone()),
        };

        // Use state.token_repo
        state.token_repo.create(&refresh_entity).await?;

        Ok(LoginResponse {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            access_token: token_data.access_token,
            refresh_token: token_data.refresh_token,
            expires_in: token_data.expires_in,
            token_type: "Bearer".to_string(),
        })
    }

    pub async fn refresh_token(
        state: web::Data<AppState>,
        token: String,
    ) -> Result<RefreshTokenResponse, AppError> {
        // Use state.token_repo
        let stored = state
            .token_repo
            .find_by_token(&token)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid session".into()))?;

        if stored.expires_at < Utc::now() || stored.revoked_at.is_some() {
            return Err(AppError::TokenExpired);
        }

        let token_data = state.keycloak_client.refresh_token(&token).await?;

        state.token_repo.revoke(&stored.token_id).await?;

        let new_token = RefreshToken {
            token_id: generate_id(TOKEN_PREFIX),
            user_id: stored.user_id,
            refresh_token: token_data.refresh_token.clone(),
            expires_at: Utc::now() + Duration::days(state.config.refresh_token_expiry_days as i64),
            created_at: Utc::now(),
            revoked_at: None,
            ip_address: stored.ip_address,
            user_agent: stored.user_agent,
        };

        state.token_repo.create(&new_token).await?;

        Ok(RefreshTokenResponse {
            access_token: token_data.access_token,
            refresh_token: token_data.refresh_token,
            expires_in: token_data.expires_in,
            token_type: "Bearer".to_string(),
        })
    }
}
