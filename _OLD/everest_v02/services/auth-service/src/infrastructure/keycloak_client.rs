use crate::core::config::Config;
use crate::core::errors::AppError;

pub struct KeycloakClient {
    pub http: reqwest::Client,
    pub config: Config,
}

impl KeycloakClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: reqwest::Client::new(),
            config,
        }
    }

    /// Fetches an Admin Token using Client Credentials flow (Service Account)
    async fn get_admin_token(&self) -> Result<String, AppError> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.keycloak_url, self.config.keycloak_realm
        );

        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.config.keycloak_backend_client_id),
            ("client_secret", &self.config.keycloak_backend_client_secret),
        ];

        let response = self
            .http
            .post(token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Keycloak Auth failed: {}", e)))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|_| AppError::Internal("Failed to parse token".into()))?;

        Ok(json["access_token"]
            .as_str()
            .unwrap_or_default()
            .to_string())
    }
}
