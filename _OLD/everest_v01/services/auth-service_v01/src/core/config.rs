use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_auth_client_id: String,
    pub keycloak_backend_client_id: String,
    pub keycloak_backend_client_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            keycloak_url: env::var("KEYCLOAK_URL")?,
            keycloak_realm: env::var("KEYCLOAK_REALM")?,
            keycloak_auth_client_id: env::var("KEYCLOAK_AUTH_CLIENT_ID")?,
            keycloak_backend_client_id: env::var("KEYCLOAK_BACKEND_CLIENT_ID")?,
            keycloak_backend_client_secret: env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")?,
        })
    }

    pub fn keycloak_issuer(&self) -> String {
        format!("{}/realms/{}", self.keycloak_url, self.keycloak_realm)
    }

    pub fn keycloak_jwks_url(&self) -> String {
        format!("{}/protocol/openid-connect/certs", self.keycloak_issuer())
    }
}