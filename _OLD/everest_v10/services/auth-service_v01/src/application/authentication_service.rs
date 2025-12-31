use crate::AppState; // Added this import
use crate::core::errors::{AppError, AppResult};
use crate::domain::services::{AuthenticationService as AuthServiceTrait, LoginResponse, UserInfo};
use crate::infrastructure::keycloak_client::KeycloakClient;
use async_trait::async_trait;
use std::sync::Arc;

pub struct AuthenticationService {
    state: Arc<AppState>,
}

impl AuthenticationService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl AuthServiceTrait for AuthenticationService {
    async fn login(&self, username: String, password: String) -> AppResult<LoginResponse> {
        // Authenticate with Keycloak via state
        let token_response = self
            .state
            .keycloak_client // Matches field name in AppState
            .authenticate(&username, &password)
            .await
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        // Get user info via state
        let user_info = self
            .state
            .keycloak_client
            .get_user_info(&token_response.access_token)
            .await
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        let email = user_info["email"]
            .as_str()
            .ok_or_else(|| AppError::Internal("No email in token".into()))?;

        // Find user in database via state repo
        let user = self
            .state
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        if !user.is_active {
            return Err(AppError::Unauthorized("Account is inactive".into()));
        }

        // Update last login
        self.state
            .user_repo
            .update_last_login(&user.user_id)
            .await?;

        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            user: UserInfo {
                user_id: user.user_id,
                email: user.email,
                username: user.username,
                role: user.role.clone(),
            },
        })
    }

    async fn logout(&self, refresh_token: String) -> AppResult<()> {
        self.state
            .keycloak_client
            .logout(&refresh_token)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn refresh_token(&self, refresh_token: String) -> AppResult<LoginResponse> {
        let token_response = self
            .state
            .keycloak_client
            .refresh_token(&refresh_token)
            .await
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        let user_info = self
            .state
            .keycloak_client
            .get_user_info(&token_response.access_token)
            .await
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        let email = user_info["email"]
            .as_str()
            .ok_or_else(|| AppError::Internal("No email in token".into()))?;

        let user = self
            .state
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            user: UserInfo {
                user_id: user.user_id,
                email: user.email,
                username: user.username,
                role: user.role.clone(),
            },
        })
    }
}
