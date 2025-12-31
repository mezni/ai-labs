use std::sync::Arc;

use crate::application::dtos::authentication::{LoginRequest, LoginResponse, ValidateResponse};
use crate::core::errors::{AppError, AppResult};
use crate::domain::repositories::UserRepository;
use crate::infrastructure::keycloak_client::KeycloakClient;

pub struct AuthenticationService {
    user_repo: Arc<dyn UserRepository>,
    keycloak_client: Arc<dyn KeycloakClient>,
}

impl AuthenticationService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        keycloak_client: Arc<dyn KeycloakClient>,
    ) -> Self {
        Self {
            user_repo,
            keycloak_client,
        }
    }

    pub async fn login(&self, req: LoginRequest) -> AppResult<LoginResponse> {
        tracing::info!("Login attempt for user: {}", req.username);

        // Authenticate with Keycloak
        let token_response = self
            .keycloak_client
            .authenticate(&req.username, &req.password)
            .await
            .map_err(|e| {
                tracing::error!("Keycloak authentication failed: {:?}", e);
                AppError::InvalidCredentials
            })?;

        // Get user info from token
        let user_info = self
            .keycloak_client
            .get_user_info(&token_response.access_token)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        let email = user_info["email"]
            .as_str()
            .ok_or(AppError::InternalError("Email not found in token".into()))?;

        let sub = user_info["sub"]
            .as_str()
            .ok_or(AppError::InternalError("Subject not found in token".into()))?;

        // Extract roles
        let roles = if let Some(roles_array) = user_info["roles"].as_array() {
            roles_array
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        } else {
            vec![]
        };

        tracing::info!("Login successful for user: {}", req.username);

        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            user_id: sub.to_string(),
            email: email.to_string(),
            roles,
        })
    }

    pub async fn logout(&self, refresh_token: &str) -> AppResult<()> {
        tracing::info!("Logout request");

        self.keycloak_client
            .logout(refresh_token)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        tracing::info!("Logout successful");
        Ok(())
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<LoginResponse> {
        tracing::info!("Token refresh request");

        let token_response = self
            .keycloak_client
            .refresh_token(refresh_token)
            .await
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        // Get user info from new token
        let user_info = self
            .keycloak_client
            .get_user_info(&token_response.access_token)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        let email = user_info["email"]
            .as_str()
            .ok_or(AppError::InternalError("Email not found in token".into()))?;

        let sub = user_info["sub"]
            .as_str()
            .ok_or(AppError::InternalError("Subject not found in token".into()))?;

        let roles = if let Some(roles_array) = user_info["roles"].as_array() {
            roles_array
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        } else {
            vec![]
        };

        tracing::info!("Token refresh successful");

        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            user_id: sub.to_string(),
            email: email.to_string(),
            roles,
        })
    }

    pub async fn validate_token(&self, access_token: &str) -> AppResult<ValidateResponse> {
        tracing::info!("Token validation request");

        let token_data = self
            .keycloak_client
            .verify_token(access_token)
            .await
            .map_err(|e| AppError::InvalidToken)?;

        let email = token_data["email"]
            .as_str()
            .ok_or(AppError::InvalidToken)?;

        let sub = token_data["sub"].as_str().ok_or(AppError::InvalidToken)?;

        let exp = token_data["exp"]
            .as_i64()
            .ok_or(AppError::InvalidToken)?;

        let roles = if let Some(roles_array) = token_data["roles"].as_array() {
            roles_array
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        } else {
            vec![]
        };

        let expires_at =
            chrono::DateTime::from_timestamp(exp, 0).unwrap_or_else(chrono::Utc::now);

        Ok(ValidateResponse {
            valid: true,
            user_id: sub.to_string(),
            email: email.to_string(),
            roles,
            expires_at,
        })
    }
}