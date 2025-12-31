use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub rust_log: String,
    pub keycloak: KeycloakConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakConfig {
    pub url: String,
    pub realm: String,
    pub auth_client_id: String,
    pub backend_client_id: String,
    pub backend_client_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()?,
            database_url: std::env::var("DATABASE_URL")?,
            rust_log: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            keycloak: KeycloakConfig {
                url: std::env::var("KEYCLOAK_URL")?,
                realm: std::env::var("KEYCLOAK_REALM")?,
                auth_client_id: std::env::var("KEYCLOAK_AUTH_CLIENT_ID")?,
                backend_client_id: std::env::var("KEYCLOAK_BACKEND_CLIENT_ID")?,
                backend_client_secret: std::env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")?,
            },
        })
    }
}
