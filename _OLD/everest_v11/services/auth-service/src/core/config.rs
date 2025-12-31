use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_auth_client_id: String,
    pub keycloak_backend_client_id: String,
    pub keycloak_backend_client_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            keycloak_url: env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set"),
            keycloak_realm: env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set"),
            keycloak_auth_client_id: env::var("KEYCLOAK_AUTH_CLIENT_ID")
                .expect("KEYCLOAK_AUTH_CLIENT_ID must be set"),
            keycloak_backend_client_id: env::var("KEYCLOAK_BACKEND_CLIENT_ID")
                .expect("KEYCLOAK_BACKEND_CLIENT_ID must be set"),
            keycloak_backend_client_secret: env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")
                .expect("KEYCLOAK_BACKEND_CLIENT_SECRET must be set"),
        }
    }
}