use crate::core::errors::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info};

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakUser {
    pub id: String,
    pub email: String,
    pub username: String,
    #[serde(default)]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    enabled: bool,
    credentials: Vec<Credential>,
    attributes: HashMap<String, Vec<String>>,
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
}

pub struct KeycloakClient {
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    client: Client,
}

impl KeycloakClient {
    pub fn new(base_url: String, realm: String, client_id: String, client_secret: String) -> Self {
        Self {
            base_url,
            realm,
            client_id,
            client_secret,
            client: Client::new(),
        }
    }

    async fn get_admin_token(&self) -> Result<String, AppError> {
        let token_url = format!("{}/realms/{}/protocol/openid-connect/token", self.base_url, self.realm);

        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let response = self
            .client
            .post(&token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to get admin token: {}", e);
                AppError::Keycloak(format!("Token request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Token request failed: {} - {}", status, body);
            return Err(AppError::Keycloak(format!("Failed to get admin token: {}", status)));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            error!("Failed to parse token response: {}", e);
            AppError::Keycloak(format!("Invalid token response: {}", e))
        })?;

        Ok(token_response.access_token)
    }

    pub async fn create_user(&self, email: &str, password: &str) -> Result<KeycloakUser, AppError> {
        let token = self.get_admin_token().await?;
        let url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);

        let user_request = CreateUserRequest {
            username: email.to_string(),
            email: email.to_string(),
            enabled: true,
            credentials: vec![Credential {
                cred_type: "password".to_string(),
                value: password.to_string(),
                temporary: false,
            }],
            attributes: HashMap::new(),
        };

        let response = self
            .client
            .post(&url)
            .bearer_auth(&token)
            .json(&user_request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to create user in Keycloak: {}", e);
                AppError::Keycloak(format!("User creation failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Keycloak user creation failed: {} - {}", status, body);
            return Err(AppError::Keycloak(format!("Failed to create user: {}", status)));
        }

        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Keycloak("No location header in response".to_string()))?;

        let user_id = location.split('/').last().unwrap_or_default().to_string();

        info!("Created user in Keycloak: {}", user_id);

        Ok(KeycloakUser {
            id: user_id,
            email: email.to_string(),
            username: email.to_string(),
            attributes: None,
        })
    }

    pub async fn set_user_attributes(
        &self,
        keycloak_id: &str,
        user_id: &str,
        roles: &[&str],
    ) -> Result<(), AppError> {
        let token = self.get_admin_token().await?;
        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.base_url, self.realm, keycloak_id
        );

        let mut attributes = HashMap::new();
        attributes.insert("user_id".to_string(), vec![user_id.to_string()]);
        attributes.insert("roles".to_string(), roles.iter().map(|r| r.to_string()).collect());

        let update_request = serde_json::json!({
            "attributes": attributes
        });

        let response = self
            .client
            .put(&url)
            .bearer_auth(&token)
            .json(&update_request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to update user attributes: {}", e);
                AppError::Keycloak(format!("Attribute update failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Failed to set attributes: {} - {}", status, body);
            return Err(AppError::Keycloak(format!("Failed to set attributes: {}", status)));
        }

        info!("Updated attributes for user: {}", keycloak_id);

        Ok(())
    }

    pub async fn list_users(&self) -> Result<Vec<KeycloakUser>, AppError> {
        let token = self.get_admin_token().await?;
        let url = format!("{}/admin/realms/{}/users?max=1000", self.base_url, self.realm);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to list users: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Keycloak("Failed to list users".to_string()));
        }

        let users: Vec<KeycloakUser> = response
            .json()
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to parse users: {}", e)))?;

        Ok(users)
    }
}