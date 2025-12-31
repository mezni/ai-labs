use anyhow::Context;

#[derive(Clone, Debug)]
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
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .context("SERVER_PORT must be a valid u16")?,
            keycloak_url: std::env::var("KEYCLOAK_URL").context("KEYCLOAK_URL must be set")?,
            keycloak_realm: std::env::var("KEYCLOAK_REALM")
                .context("KEYCLOAK_REALM must be set")?,
            keycloak_auth_client_id: std::env::var("KEYCLOAK_AUTH_CLIENT_ID")
                .context("KEYCLOAK_AUTH_CLIENT_ID must be set")?,
            keycloak_backend_client_id: std::env::var("KEYCLOAK_BACKEND_CLIENT_ID")
                .context("KEYCLOAK_BACKEND_CLIENT_ID must be set")?,
            keycloak_backend_client_secret: std::env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")
                .context("KEYCLOAK_BACKEND_CLIENT_SECRET must be set")?,
        })
    }

    pub fn token_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.keycloak_url, self.keycloak_realm
        )
    }

    pub fn backend_token_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.keycloak_url, self.keycloak_realm
        )
    }

    pub fn users_url(&self) -> String {
        format!(
            "{}/admin/realms/{}/users",
            self.keycloak_url, self.keycloak_realm
        )
    }

    pub fn user_url(&self, user_id: &str) -> String {
        format!("{}/{}", self.users_url(), user_id)
    }

    pub fn roles_url(&self) -> String {
        format!(
            "{}/admin/realms/{}/roles",
            self.keycloak_url, self.keycloak_realm
        )
    }
}
