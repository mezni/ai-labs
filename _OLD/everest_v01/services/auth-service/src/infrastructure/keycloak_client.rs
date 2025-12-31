// src/infrastructure/keycloak_client.rs
use crate::core::{AppError, config::KeycloakConfig, errors::AppResult};
use crate::domain::TokenResponse;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Clone)]
pub struct KeycloakClient {
    config: KeycloakConfig,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakUser {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    pub enabled: bool,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    pub enabled: bool,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub credentials: Option<Vec<UserCredential>>,
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredential {
    #[serde(rename = "type")]
    pub credential_type: String,
    pub value: String,
    pub temporary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleRepresentation {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

impl KeycloakClient {
    pub fn new(config: KeycloakConfig) -> Self {
        Self {
            config,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Get admin access token
    async fn get_admin_token(&self) -> AppResult<String> {
        let params = [
            ("client_id", self.config.backend_client_id.as_str()),
            ("client_secret", self.config.backend_client_secret.as_str()),
            ("grant_type", "client_credentials"),
        ];

        let response = self
            .client
            .post(&self.config.token_endpoint())
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to get admin token: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Keycloak(format!(
                "Failed to get admin token: {} - {}",
                status, text
            )));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            AppError::Keycloak(format!("Failed to parse admin token response: {}", e))
        })?;

        Ok(token_response.access_token)
    }

    /// Authenticate user with username/password
    pub async fn login(&self, username: &str, password: &str) -> AppResult<TokenResponse> {
        let params = [
            ("client_id", self.config.auth_client_id.as_str()),
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
        ];

        let response = self
            .client
            .post(&self.config.token_endpoint())
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Login request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to parse token response: {}", e)))?;

        Ok(token_response)
    }

    /// Refresh access token
    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<TokenResponse> {
        let params = [
            ("client_id", self.config.auth_client_id.as_str()),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];

        let response = self
            .client
            .post(&self.config.token_endpoint())
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Token refresh failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Unauthorized("Invalid refresh token".to_string()));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to parse token response: {}", e)))?;

        Ok(token_response)
    }

    /// Create a new user in Keycloak
    pub async fn create_user(&self, request: CreateUserRequest) -> AppResult<String> {
        let admin_token = self.get_admin_token().await?;

        let response = self
            .client
            .post(&self.config.user_endpoint())
            .bearer_auth(&admin_token)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to create user: {}", e)))?;

        if response.status() != StatusCode::CREATED {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Keycloak(format!(
                "Failed to create user: {} - {}",
                status, text
            )));
        }

        // Extract user ID from Location header
        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Keycloak("No Location header in response".to_string()))?;

        let user_id = location
            .split('/')
            .last()
            .ok_or_else(|| AppError::Keycloak("Invalid Location header".to_string()))?
            .to_string();

        Ok(user_id)
    }

    /// Get user by ID
    pub async fn get_user(&self, keycloak_id: &str) -> AppResult<KeycloakUser> {
        let admin_token = self.get_admin_token().await?;
        let url = format!("{}/{}", self.config.user_endpoint(), keycloak_id);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&admin_token)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to get user: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::NotFound("User not found in Keycloak".to_string()));
        }

        let user: KeycloakUser = response
            .json()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to parse user response: {}", e)))?;

        Ok(user)
    }

    /// Update user in Keycloak
    pub async fn update_user(
        &self,
        keycloak_id: &str,
        email: Option<String>,
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        enabled: Option<bool>,
    ) -> AppResult<()> {
        let admin_token = self.get_admin_token().await?;
        let url = format!("{}/{}", self.config.user_endpoint(), keycloak_id);

        let mut update_data = json!({});

        if let Some(e) = email {
            update_data["email"] = json!(e);
        }
        if let Some(u) = username {
            update_data["username"] = json!(u);
        }
        if let Some(f) = first_name {
            update_data["firstName"] = json!(f);
        }
        if let Some(l) = last_name {
            update_data["lastName"] = json!(l);
        }
        if let Some(en) = enabled {
            update_data["enabled"] = json!(en);
        }

        let response = self
            .client
            .put(&url)
            .bearer_auth(&admin_token)
            .json(&update_data)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to update user: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Keycloak(format!(
                "Failed to update user: {} - {}",
                status, text
            )));
        }

        Ok(())
    }

    /// Delete/disable user in Keycloak
    pub async fn disable_user(&self, keycloak_id: &str) -> AppResult<()> {
        self.update_user(keycloak_id, None, None, None, None, Some(false))
            .await
    }

    /// Set user password
    pub async fn set_password(
        &self,
        keycloak_id: &str,
        password: &str,
        temporary: bool,
    ) -> AppResult<()> {
        let admin_token = self.get_admin_token().await?;
        let url = format!(
            "{}/{}/reset-password",
            self.config.user_endpoint(),
            keycloak_id
        );

        let credential = UserCredential {
            credential_type: "password".to_string(),
            value: password.to_string(),
            temporary,
        };

        let response = self
            .client
            .put(&url)
            .bearer_auth(&admin_token)
            .json(&credential)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to set password: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Keycloak(format!(
                "Failed to set password: {} - {}",
                status, text
            )));
        }

        Ok(())
    }

    /// Send password reset email
    pub async fn send_password_reset_email(&self, keycloak_id: &str) -> AppResult<()> {
        let admin_token = self.get_admin_token().await?;
        let url = format!(
            "{}/{}/execute-actions-email",
            self.config.user_endpoint(),
            keycloak_id
        );

        let actions = vec!["UPDATE_PASSWORD"];

        let response = self
            .client
            .put(&url)
            .bearer_auth(&admin_token)
            .json(&actions)
            .send()
            .await
            .map_err(|e| {
                AppError::Keycloak(format!("Failed to send password reset email: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Keycloak(format!(
                "Failed to send password reset email: {} - {}",
                status, text
            )));
        }

        Ok(())
    }

    /// Assign role to user
    pub async fn assign_role(&self, keycloak_id: &str, role: &str) -> AppResult<()> {
        let admin_token = self.get_admin_token().await?;

        // Get available realm roles
        let roles_url = format!(
            "{}/admin/realms/{}/roles",
            self.config.url, self.config.realm
        );

        let response = self
            .client
            .get(&roles_url)
            .bearer_auth(&admin_token)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to get roles: {}", e)))?;

        let roles: Vec<RoleRepresentation> = response
            .json()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to parse roles: {}", e)))?;

        let role_repr = roles
            .into_iter()
            .find(|r| r.name == role)
            .ok_or_else(|| AppError::Keycloak(format!("Role '{}' not found", role)))?;

        // Assign role to user
        let assign_url = format!(
            "{}/{}/role-mappings/realm",
            self.config.user_endpoint(),
            keycloak_id
        );

        let response = self
            .client
            .post(&assign_url)
            .bearer_auth(&admin_token)
            .json(&vec![role_repr])
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to assign role: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Keycloak(format!(
                "Failed to assign role: {} - {}",
                status, text
            )));
        }

        Ok(())
    }

    /// Send verification email
    pub async fn send_verification_email(&self, keycloak_id: &str) -> AppResult<()> {
        let admin_token = self.get_admin_token().await?;
        let url = format!(
            "{}/{}/send-verify-email",
            self.config.user_endpoint(),
            keycloak_id
        );

        let response = self
            .client
            .put(&url)
            .bearer_auth(&admin_token)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to send verification email: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Keycloak(format!(
                "Failed to send verification email: {} - {}",
                status, text
            )));
        }

        Ok(())
    }
}
