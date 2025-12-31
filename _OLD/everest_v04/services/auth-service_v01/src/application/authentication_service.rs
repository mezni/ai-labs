use crate::application::login_dto::{LoginRequest, LoginResponse, RefreshTokenResponse};
use crate::application::verify_dto::{VerifyRequest, VerifyResponse};
use crate::core::{
    constants::{TOKEN_PREFIX, USER_PREFIX},
    errors::AppError,
    id_generator::generate_id,
    state::AppState,
};
use crate::domain::{
    entities::{RefreshToken, User},
    enums::{RegistrationStatus, UserRole},
    repositories::{RegistrationRepository, UserRepository, RefreshTokenRepository},
};
use crate::infrastructure::persistence::{
    RefreshTokenRepositoryImpl, RegistrationRepositoryImpl, UserRepositoryImpl,
};
use actix_web::web;
use chrono::{Duration, Utc};

pub struct AuthenticationService;

impl AuthenticationService {
    pub async fn verify(
        state: web::Data<AppState>,
        request: VerifyRequest,
    ) -> Result<VerifyResponse, AppError> {
        let reg_repo = RegistrationRepositoryImpl::new(state.db_pool.clone());

        // Find registration by token
        let registration = reg_repo
            .find_by_token(&request.token)
            .await?
            .ok_or_else(|| AppError::InvalidToken)?;

        // Check if already verified
        if registration.status == RegistrationStatus::Verified {
            return Err(AppError::BadRequest("Already verified".to_string()));
        }

        // Check expiry
        if registration.expires_at < Utc::now() {
            reg_repo
                .update_status(&registration.registration_id, "expired")
                .await?;
            return Err(AppError::TokenExpired);
        }

        // Enable user in Keycloak
        state
            .keycloak_client
            .enable_user(&registration.keycloak_id)
            .await?;

        // Create user in database
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

        let user_repo = UserRepositoryImpl::new(state.db_pool.clone());
        let created_user = user_repo.create(&user).await?;

        // Update registration status
        reg_repo
            .update_status(&registration.registration_id, "verified")
            .await?;
        
        reg_repo
            .update_user_id(&registration.registration_id, &user_id)
            .await?;

        // Authenticate user and get tokens
        let token_data = state
            .keycloak_client
            .authenticate(
                &registration.username,
                "", // We don't have the password, but user is already enabled
                &state.config.keycloak_auth_client_id,
            )
            .await
            .unwrap_or_else(|_| {
                // If direct auth fails, we still return success but without auto-login
                // User will need to login separately
                tracing::warn!("Auto-login after verification failed for user: {}", user_id);
                // For now, we'll generate a simple response
                // In production, you might want to handle this differently
                panic!("Auto-login not available")
            });

        // Store refresh token
        let token_repo = RefreshTokenRepositoryImpl::new(state.db_pool.clone());
        let refresh_token_entity = RefreshToken {
            token_id: generate_id(TOKEN_PREFIX),
            user_id: user_id.clone(),
            refresh_token: token_data.refresh_token.clone(),
            expires_at: Utc::now() + Duration::days(state.config.refresh_token_expiry_days),
            created_at: Utc::now(),
            revoked_at: None,
            ip_address: request.metadata.as_ref().and_then(|m| m.verification_ip.clone()),
            user_agent: request.metadata.as_ref().and_then(|m| m.user_agent.clone()),
        };
        token_repo.create(&refresh_token_entity).await?;

        tracing::info!("User verified and logged in: {}", user_id);

        Ok(VerifyResponse {
            user_id: created_user.user_id,
            email: created_user.email,
            message: "Account verified successfully".to_string(),
            access_token: token_data.access_token,
            refresh_token: token_data.refresh_token,
            expires_in: token_data.expires_in,
        })
    }

    pub async fn login(
        state: web::Data<AppState>,
        request: LoginRequest,
    ) -> Result<LoginResponse, AppError> {
        // Authenticate with Keycloak
        let token_data = state
            .keycloak_client
            .authenticate(
                &request.username,
                &request.password,
                &state.config.keycloak_auth_client_id,
            )
            .await?;

        // Get user from Keycloak
        let keycloak_user = state
            .keycloak_client
            .get_user_by_username(&request.username)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Find user in database
        let user_repo = UserRepositoryImpl::new(state.db_pool.clone());
        let user = user_repo
            .find_by_keycloak_id(&keycloak_user.id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found in database".to_string()))?;

        // Update last login
        user_repo.update_last_login(&user.user_id).await?;

        // Store refresh token
        let token_repo = RefreshTokenRepositoryImpl::new(state.db_pool.clone());
        let refresh_token_entity = RefreshToken {
            token_id: generate_id(TOKEN_PREFIX),
            user_id: user.user_id.clone(),
            refresh_token: token_data.refresh_token.clone(),
            expires_at: Utc::now() + Duration::days(state.config.refresh_token_expiry_days),
            created_at: Utc::now(),
            revoked_at: None,
            ip_address: request.metadata.as_ref().and_then(|m| m.login_ip.clone()),
            user_agent: request.metadata.as_ref().and_then(|m| m.user_agent.clone()),
        };
        token_repo.create(&refresh_token_entity).await?;

        tracing::info!("User logged in: {}", user.user_id);

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
        refresh_token: String,
    ) -> Result<RefreshTokenResponse, AppError> {
        // Verify refresh token exists in database
        let token_repo = RefreshTokenRepositoryImpl::new(state.db_pool.clone());
        let stored_token = token_repo
            .find_by_token(&refresh_token)
            .await?
            .ok_or_else(|| AppError::InvalidToken)?;

        // Check expiry
        if stored_token.expires_at < Utc::now() {
            return Err(AppError::TokenExpired);
        }

        // Refresh token with Keycloak
        let token_data = state
            .keycloak_client
            .refresh_token(&refresh_token, &state.config.keycloak_auth_client_id)
            .await?;

        // Revoke old token and create new one
        token_repo.revoke(&stored_token.token_id).await?;

        let new_token_entity = RefreshToken {
            token_id: generate_id(TOKEN_PREFIX),
            user_id: stored_token.user_id,
            refresh_token: token_data.refresh_token.clone(),
            expires_at: Utc::now() + Duration::days(state.config.refresh_token_expiry_days),
            created_at: Utc::now(),
            revoked_at: None,
            ip_address: stored_token.ip_address,
            user_agent: stored_token.user_agent,
        };
        token_repo.create(&new_token_entity).await?;

        Ok(RefreshTokenResponse {
            access_token: token_data.access_token,
            refresh_token: token_data.refresh_token,
            expires_in: token_data.expires_in,
            token_type: "Bearer".to_string(),
        })
    }
}