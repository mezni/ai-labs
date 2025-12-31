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
    pub verification_expiry_hours: i64,
    pub access_token_expiry_secs: i64,
    pub refresh_token_expiry_days: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            keycloak_url: env::var("KEYCLOAK_URL")?,
            keycloak_realm: env::var("KEYCLOAK_REALM")?,
            keycloak_auth_client_id: env::var("KEYCLOAK_AUTH_CLIENT_ID")?,
            keycloak_backend_client_id: env::var("KEYCLOAK_BACKEND_CLIENT_ID")?,
            keycloak_backend_client_secret: env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")?,
            verification_expiry_hours: env::var("VERIFICATION_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            access_token_expiry_secs: env::var("ACCESS_TOKEN_EXPIRY_SECS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
            refresh_token_expiry_days: env::var("REFRESH_TOKEN_EXPIRY_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .unwrap_or(7),
        })
    }
}