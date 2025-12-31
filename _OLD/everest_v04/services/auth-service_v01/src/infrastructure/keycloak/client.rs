use crate::core::errors::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone)]
pub struct KeycloakClient {
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    http_client: Client,
}

#[derive(Debug, Serialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
    enabled: bool,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    credentials: Vec<Credential>,
    attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize)]
struct Credential {
    #[serde(rename = "type")]
    cred_type: String,
    value: String,
    temporary: bool,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
    #[serde(rename = "token_type")]
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct UserRepresentation {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

impl KeycloakClient {
    pub fn new(
        base_url: String,
        realm: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            base_url,
            realm,
            client_id,
            client_secret,
            http_client: Client::new(),
        }
    }

    async fn get_admin_token(&self) -> Result<String, AppError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "client_credentials"),
        ];

        let response = self
            .http_client
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Failed to get admin token: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Failed to get admin token: {}",
                error_text
            )));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            AppError::KeycloakError(format!("Failed to parse token response: {}", e))
        })?;

        Ok(token_response.access_token)
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        first_name: Option<String>,
        last_name: Option<String>,
        verification_token: &str,
    ) -> Result<String, AppError> {
        let admin_token = self.get_admin_token().await?;

        let url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);

        let mut attributes = HashMap::new();
        attributes.insert(
            "verification_token".to_string(),
            vec![verification_token.to_string()],
        );

        let user_request = CreateUserRequest {
            username: username.to_string(),
            email: email.to_string(),
            first_name,
            last_name,
            enabled: false,
            email_verified: false,
            credentials: vec![Credential {
                cred_type: "password".to_string(),
                value: password.to_string(),
                temporary: false,
            }],
            attributes: Some(attributes),
        };

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&admin_token)
            .json(&user_request)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Failed to create user: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Failed to create user in Keycloak: {}",
                error_text
            )));
        }

        // Extract user ID from Location header
        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                AppError::KeycloakError("No Location header in create user response".to_string())
            })?;

        let user_id = location
            .split('/')
            .last()
            .ok_or_else(|| {
                AppError::KeycloakError("Invalid Location header format".to_string())
            })?
            .to_string();

        Ok(user_id)
    }

    pub async fn enable_user(&self, keycloak_user_id: &str) -> Result<(), AppError> {
        let admin_token = self.get_admin_token().await?;

        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, keycloak_user_id
        );

        let update_data = serde_json::json!({
            "enabled": true,
            "emailVerified": true
        });

        let response = self
            .http_client
            .put(&url)
            .bearer_auth(&admin_token)
            .json(&update_data)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Failed to enable user: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Failed to enable user: {}",
                error_text
            )));
        }

        Ok(())
    }

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
        auth_client_id: &str,
    ) -> Result<TokenData, AppError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let params = [
            ("client_id", auth_client_id),
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
        ];

        let response = self
            .http_client
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Authentication failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Unauthorized(format!(
                "Invalid credentials: {}",
                error_text
            )));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            AppError::KeycloakError(format!("Failed to parse token response: {}", e))
        })?;

        Ok(TokenData {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_in: token_response.expires_in,
        })
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<UserRepresentation>, AppError> {
        let admin_token = self.get_admin_token().await?;

        let url = format!(
            "{}/admin/realms/{}/users?username={}&exact=true",
            self.base_url, self.realm, username
        );

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&admin_token)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Failed to get user: {}", e)))?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let users: Vec<UserRepresentation> = response.json().await.map_err(|e| {
            AppError::KeycloakError(format!("Failed to parse user response: {}", e))
        })?;

        Ok(users.into_iter().next())
    }

    pub async fn refresh_token(
        &self,
        refresh_token: &str,
        auth_client_id: &str,
    ) -> Result<TokenData, AppError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let params = [
            ("client_id", auth_client_id),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];

        let response = self
            .http_client
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::KeycloakError(format!("Token refresh failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Unauthorized(format!(
                "Invalid refresh token: {}",
                error_text
            )));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            AppError::KeycloakError(format!("Failed to parse token response: {}", e))
        })?;

        Ok(TokenData {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_in: token_response.expires_in,
        })
    }

    pub async fn send_verification_email(&self, keycloak_user_id: &str) -> Result<(), AppError> {
        let admin_token = self.get_admin_token().await?;

        let url = format!(
            "{}/admin/realms/{}/users/{}/execute-actions-email",
            self.base_url, self.realm, keycloak_user_id
        );

        let actions = vec!["VERIFY_EMAIL"];

        let response = self
            .http_client
            .put(&url)
            .bearer_auth(&admin_token)
            .json(&actions)
            .send()
            .await
            .map_err(|e| {
                AppError::KeycloakError(format!("Failed to send verification email: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            tracing::warn!("Failed to send verification email: {}", error_text);
            // Don't fail the registration if email sending fails
        }

        Ok(())
    }
}