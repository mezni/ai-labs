use crate::core::constants::*;
use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
    pub log_level: String,

    // Keycloak
    pub keycloak_url: String,
    pub keycloak_realm: String,

    // Backend (admin)
    pub keycloak_backend_client_id: String,
    pub keycloak_backend_client_secret: String,

    // Public auth
    pub keycloak_auth_client_id: String,

    pub verification_expiry_hours: u64,
    pub refresh_token_expiry_days: u64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
        let port = env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());

        Self {
            server_addr: format!("{host}:{port}"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),

            log_level: env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string()),

            keycloak_url: env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set"),
            keycloak_realm: env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set"),

            keycloak_backend_client_id: env::var("KEYCLOAK_BACKEND_CLIENT_ID")
                .expect("KEYCLOAK_BACKEND_CLIENT_ID must be set"),
            keycloak_backend_client_secret: env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")
                .expect("KEYCLOAK_BACKEND_CLIENT_SECRET must be set"),

            keycloak_auth_client_id: env::var("KEYCLOAK_AUTH_CLIENT_ID")
                .expect("KEYCLOAK_AUTH_CLIENT_ID must be set"),

            verification_expiry_hours: env::var("VERIFICATION_EXPIRY_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_VERIFICATION_EXPIRY_HOURS),

            refresh_token_expiry_days: env::var("REFRESH_TOKEN_EXPIRY_DAYS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_REFRESH_TOKEN_EXPIRY_DAYS),
        }
    }
}
