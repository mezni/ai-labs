use crate::core::constants::{DEFAULT_HOST, DEFAULT_LOG_LEVEL, DEFAULT_PORT};
use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
    pub log_level: String,

    // Keycloak Base
    pub keycloak_url: String,
    pub keycloak_realm: String,

    // Management Client (Admin Tasks)
    pub keycloak_backend_client_id: String,
    pub keycloak_backend_client_secret: String,

    // Public Client (User Auth)
    pub keycloak_auth_client_id: String,

}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            server_addr: format!(
                "{}:{}",
                env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string()),
                env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string())
            ),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string()),

            keycloak_url: std::env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set"),
            keycloak_realm: std::env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set"),

            keycloak_backend_client_id: std::env::var("KEYCLOAK_BACKEND_CLIENT_ID")
                .expect("BACKEND_CLIENT_ID must be set"),
            keycloak_backend_client_secret: std::env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")
                .expect("BACKEND_CLIENT_SECRET must be set"),

            keycloak_auth_client_id: std::env::var("KEYCLOAK_AUTH_CLIENT_ID")
                .expect("AUTH_CLIENT_ID must be set"),


        }
    }
}
