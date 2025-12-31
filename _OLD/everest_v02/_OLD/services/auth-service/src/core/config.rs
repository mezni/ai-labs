use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub database_max_connections: u32,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_auth_client_id: String,
    pub keycloak_backend_client_id: String,
    pub keycloak_backend_client_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL")?,
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            keycloak_url: env::var("KEYCLOAK_URL")?,
            keycloak_realm: env::var("KEYCLOAK_REALM")?,
            keycloak_auth_client_id: env::var("KEYCLOAK_AUTH_CLIENT_ID")?,
            keycloak_backend_client_id: env::var("KEYCLOAK_BACKEND_CLIENT_ID")?,
            keycloak_backend_client_secret: env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")?,
        })
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.keycloak_backend_client_secret.is_empty() {
            anyhow::bail!("KEYCLOAK_BACKEND_CLIENT_SECRET cannot be empty");
        }
        if self.database_url.is_empty() {
            anyhow::bail!("DATABASE_URL cannot be empty");
        }
        Ok(())
    }
}
