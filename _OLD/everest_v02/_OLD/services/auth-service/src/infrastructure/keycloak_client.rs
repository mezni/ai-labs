use crate::core::errors::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct KeycloakClient {
    client: Client,
    base_url: String,
    realm: String,
    auth_client_id: String,
    backend_client_id: String,
    backend_client_secret: String,
    token_cache: Arc<RwLock<Option<TokenCache>>>,
}

#[derive(Clone)]
struct TokenCache {
    access_token: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    token_type: Option<String>,
}

#[derive(Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub enabled: bool,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(rename = "requiredActions", skip_serializing_if = "Option::is_none")]
    pub required_actions: Option<Vec<String>>,
}

#[derive(Serialize)]
struct UpdateUserRequest {
    enabled: bool,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
}

impl KeycloakClient {
    pub fn new(
        base_url: String,
        realm: String,
        auth_client_id: String,
        backend_client_id: String,
        backend_client_secret: String,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url,
            realm,
            auth_client_id,
            backend_client_id,
            backend_client_secret,
            token_cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Get service account token for backend-admin client
    async fn get_backend_token(&self) -> Result<String, AppError> {
        // Check cache first
        {
            let cache = self.token_cache.read().await;
            if let Some(ref cached) = *cache {
                // Add 60 second buffer before expiration
                if cached.expires_at > chrono::Utc::now() + chrono::Duration::seconds(60) {
                    return Ok(cached.access_token.clone());
                }
            }
        }

        // Get new token
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.backend_client_id),
            ("client_secret", &self.backend_client_secret),
        ];

        tracing::debug!("Requesting backend token from Keycloak");

        let response = self
            .client
            .post(&token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Token request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Token request failed with status {}: {}",
                status, body
            )));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            AppError::KeycloakError(format!("Failed to parse token response: {}", e))
        })?;

        // Cache the token
        {
            let mut cache = self.token_cache.write().await;
            *cache = Some(TokenCache {
                access_token: token_response.access_token.clone(),
                expires_at: chrono::Utc::now()
                    + chrono::Duration::seconds(token_response.expires_in - 60),
            });
        }

        tracing::debug!("Backend token obtained successfully");
        Ok(token_response.access_token)
    }

    /// Create a new user in Keycloak
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<String, AppError> {
        let token = self.get_backend_token().await?;
        let url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);

        tracing::debug!("Creating user in Keycloak: {}", request.email);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&token)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Create user request failed: {}", e)))?;

        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();

            if status.as_u16() == 409 {
                return Err(AppError::Conflict(
                    "User with this username or email already exists in Keycloak".to_string(),
                ));
            }

            return Err(AppError::KeycloakError(format!(
                "Create user failed with status {}: {}",
                status, body
            )));
        }

        // Extract user ID from Location header
        let location = response
            .headers()
            .get("location")
            .ok_or_else(|| AppError::KeycloakError("No Location header in response".to_string()))?
            .to_str()
            .map_err(|e| AppError::KeycloakError(format!("Invalid Location header: {}", e)))?;

        let user_id = location
            .split('/')
            .last()
            .ok_or_else(|| AppError::KeycloakError("Invalid user ID in Location".to_string()))?
            .to_string();

        tracing::info!("User created in Keycloak with ID: {}", user_id);
        Ok(user_id)
    }

    /// Enable a user and mark email as verified
    pub async fn enable_user(&self, keycloak_id: &str) -> Result<(), AppError> {
        let token = self.get_backend_token().await?;
        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, keycloak_id
        );

        tracing::debug!("Enabling user in Keycloak: {}", keycloak_id);

        let update_request = UpdateUserRequest {
            enabled: true,
            email_verified: true,
        };

        let response = self
            .client
            .put(&url)
            .bearer_auth(&token)
            .json(&update_request)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Enable user request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Enable user failed with status {}: {}",
                status, body
            )));
        }

        tracing::info!("User enabled in Keycloak: {}", keycloak_id);
        Ok(())
    }

    /// Send verification email to user
    pub async fn send_verify_email(&self, keycloak_id: &str) -> Result<(), AppError> {
        let token = self.get_backend_token().await?;
        let url = format!(
            "{}/admin/realms/{}/users/{}/execute-actions-email",
            self.base_url, self.realm, keycloak_id
        );

        tracing::debug!("Sending verification email via Keycloak: {}", keycloak_id);

        let response = self
            .client
            .put(&url)
            .bearer_auth(&token)
            .json(&vec!["VERIFY_EMAIL"])
            .send()
            .await
            .map_err(|e| {
                AppError::KeycloakError(format!("Send verification email failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Send verification email failed with status {}: {}",
                status, body
            )));
        }

        tracing::info!(
            "Verification email sent via Keycloak for user: {}",
            keycloak_id
        );
        Ok(())
    }

    /// Get user details from Keycloak
    pub async fn get_user(&self, keycloak_id: &str) -> Result<serde_json::Value, AppError> {
        let token = self.get_backend_token().await?;
        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, keycloak_id
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Get user request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            if status.as_u16() == 404 {
                return Err(AppError::NotFound("User not found in Keycloak".to_string()));
            }
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Get user failed with status {}: {}",
                status, body
            )));
        }

        let user: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Failed to parse user: {}", e)))?;

        Ok(user)
    }

    /// Disable a user in Keycloak
    pub async fn disable_user(&self, keycloak_id: &str) -> Result<(), AppError> {
        let token = self.get_backend_token().await?;
        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, keycloak_id
        );

        tracing::debug!("Disabling user in Keycloak: {}", keycloak_id);

        let response = self
            .client
            .put(&url)
            .bearer_auth(&token)
            .json(&json!({ "enabled": false }))
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Disable user request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Disable user failed with status {}: {}",
                status, body
            )));
        }

        tracing::info!("User disabled in Keycloak: {}", keycloak_id);
        Ok(())
    }

    /// Delete a user from Keycloak
    pub async fn delete_user(&self, keycloak_id: &str) -> Result<(), AppError> {
        let token = self.get_backend_token().await?;
        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, keycloak_id
        );

        tracing::debug!("Deleting user from Keycloak: {}", keycloak_id);

        let response = self
            .client
            .delete(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Delete user request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Delete user failed with status {}: {}",
                status, body
            )));
        }

        tracing::info!("User deleted from Keycloak: {}", keycloak_id);
        Ok(())
    }
}
