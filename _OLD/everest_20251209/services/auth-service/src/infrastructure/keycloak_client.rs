use crate::config::Config;
use crate::domain::{KeycloakTokenResponse, KeycloakUser, UserRole};
use crate::infrastructure::error::DomainError;
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;

pub struct KeycloakClient {
    client: Client,
    config: Config,
}

#[derive(Serialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    enabled: bool,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    credentials: Vec<CredentialRepresentation>,
    attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize)]
struct CredentialRepresentation {
    r#type: String,
    value: String,
    temporary: bool,
}

#[derive(Serialize)]
struct PasswordResetRequest {
    r#type: String,
    value: String,
    temporary: bool,
}

impl KeycloakClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Get service account token for backend-admin client
    async fn get_backend_token(&self) -> Result<String, DomainError> {
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.config.keycloak_backend_client_id),
            ("client_secret", &self.config.keycloak_backend_client_secret),
        ];

        let response = self
            .client
            .post(&self.config.backend_token_url())
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                DomainError::KeycloakError(format!("Failed to get backend token: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(DomainError::KeycloakError(format!(
                "Backend token request failed with status {}: {}",
                status, text
            )));
        }

        let token_response: KeycloakTokenResponse = response
            .json()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to parse token: {}", e)))?;

        Ok(token_response.access_token)
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        role: UserRole,
        network_id: Option<&str>,
        station_id: Option<&str>,
    ) -> Result<String, DomainError> {
        let backend_token = self.get_backend_token().await?;

        // Prepare attributes
        let mut attributes = HashMap::new();
        if let Some(nid) = network_id {
            attributes.insert("network_id".to_string(), vec![nid.to_string()]);
        }
        if let Some(sid) = station_id {
            attributes.insert("station_id".to_string(), vec![sid.to_string()]);
        }

        let create_request = CreateUserRequest {
            username: username.to_string(),
            email: email.to_string(),
            enabled: true,
            email_verified: true,
            first_name: first_name.map(|s| s.to_string()),
            last_name: last_name.map(|s| s.to_string()),
            credentials: vec![CredentialRepresentation {
                r#type: "password".to_string(),
                value: password.to_string(),
                temporary: false,
            }],
            attributes: if attributes.is_empty() {
                None
            } else {
                Some(attributes)
            },
        };

        let response = self
            .client
            .post(&self.config.users_url())
            .bearer_auth(&backend_token)
            .json(&create_request)
            .send()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to create user: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            if status.as_u16() == 409 {
                return Err(DomainError::UserAlreadyExists(
                    "User with this username or email already exists in Keycloak".to_string(),
                ));
            }

            return Err(DomainError::KeycloakError(format!(
                "User creation failed with status {}: {}",
                status, text
            )));
        }

        // Extract user ID from Location header
        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                DomainError::KeycloakError("No Location header in response".to_string())
            })?;

        let user_id = location
            .split('/')
            .last()
            .ok_or_else(|| DomainError::KeycloakError("Invalid Location header".to_string()))?
            .to_string();

        // Assign role to user
        self.assign_role_to_user(&user_id, role, &backend_token)
            .await?;

        Ok(user_id)
    }

    async fn assign_role_to_user(
        &self,
        user_id: &str,
        role: UserRole,
        backend_token: &str,
    ) -> Result<(), DomainError> {
        let role_name = role.as_str();
        let role_url = format!("{}/{}", self.config.roles_url(), role_name);

        // Get role representation
        let role_response = self
            .client
            .get(&role_url)
            .bearer_auth(backend_token)
            .send()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to get role: {}", e)))?;

        if !role_response.status().is_success() {
            tracing::warn!(
                "Role {} not found in Keycloak, skipping role assignment",
                role_name
            );
            return Ok(());
        }

        let role_repr: serde_json::Value = role_response
            .json()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to parse role: {}", e)))?;

        // Assign role to user
        let assign_url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm",
            self.config.keycloak_url, self.config.keycloak_realm, user_id
        );

        let response = self
            .client
            .post(&assign_url)
            .bearer_auth(backend_token)
            .json(&vec![role_repr])
            .send()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to assign role: {}", e)))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            tracing::warn!("Role assignment failed: {}", text);
        }

        Ok(())
    }

    /// Login using auth-client (public client for user authentication)
    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<KeycloakTokenResponse, DomainError> {
        let params = [
            ("grant_type", "password"),
            ("client_id", self.config.keycloak_auth_client_id.as_str()),
            ("username", username),
            ("password", password),
        ];

        let response = self
            .client
            .post(&self.config.token_url())
            .form(&params)
            .send()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Login request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(DomainError::AuthenticationError(format!(
                "Login failed with status {}: {}",
                status, text
            )));
        }

        let token_response: KeycloakTokenResponse = response
            .json()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to parse token: {}", e)))?;

        Ok(token_response)
    }

    pub async fn change_password(
        &self,
        user_keycloak_id: &str,
        new_password: &str,
    ) -> Result<(), DomainError> {
        let backend_token = self.get_backend_token().await?;

        let password_request = PasswordResetRequest {
            r#type: "password".to_string(),
            value: new_password.to_string(),
            temporary: false,
        };

        let url = format!(
            "{}/admin/realms/{}/users/{}/reset-password",
            self.config.keycloak_url, self.config.keycloak_realm, user_keycloak_id
        );

        let response = self
            .client
            .put(&url)
            .bearer_auth(&backend_token)
            .json(&password_request)
            .send()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Password change failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(DomainError::KeycloakError(format!(
                "Password change failed with status {}: {}",
                status, text
            )));
        }

        Ok(())
    }

    /// Refresh token using auth-client
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<KeycloakTokenResponse, DomainError> {
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", self.config.keycloak_auth_client_id.as_str()),
            ("refresh_token", refresh_token),
        ];

        let response = self
            .client
            .post(&self.config.token_url())
            .form(&params)
            .send()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Token refresh failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(DomainError::AuthenticationError(
                "Token refresh failed".to_string(),
            ));
        }

        let token_response: KeycloakTokenResponse = response
            .json()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to parse token: {}", e)))?;

        Ok(token_response)
    }

    pub async fn get_user(&self, user_id: &str) -> Result<KeycloakUser, DomainError> {
        let backend_token = self.get_backend_token().await?;

        let response = self
            .client
            .get(&self.config.user_url(user_id))
            .bearer_auth(&backend_token)
            .send()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to get user: {}", e)))?;

        if !response.status().is_success() {
            return Err(DomainError::NotFound(
                "User not found in Keycloak".to_string(),
            ));
        }

        let user: KeycloakUser = response
            .json()
            .await
            .map_err(|e| DomainError::KeycloakError(format!("Failed to parse user: {}", e)))?;

        Ok(user)
    }
}
