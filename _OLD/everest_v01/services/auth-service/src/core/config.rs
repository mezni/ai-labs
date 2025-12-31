// src/core/config.rs
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub keycloak: KeycloakConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub issuer: String,
    pub jwks_url: String,
    pub cache_duration_secs: u64,
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
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|e| format!("Invalid SERVER_PORT: {}", e))?,
        };

        let database = DatabaseConfig {
            url: env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set".to_string())?,
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        };

        let jwt = JwtConfig {
            issuer: env::var("JWT_ISSUER").map_err(|_| "JWT_ISSUER must be set".to_string())?,
            jwks_url: env::var("JWKS_URL").map_err(|_| "JWKS_URL must be set".to_string())?,
            cache_duration_secs: env::var("JWT_CACHE_DURATION_SECS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
        };

        let keycloak = KeycloakConfig {
            url: env::var("KEYCLOAK_URL").map_err(|_| "KEYCLOAK_URL must be set".to_string())?,
            realm: env::var("KEYCLOAK_REALM")
                .map_err(|_| "KEYCLOAK_REALM must be set".to_string())?,
            auth_client_id: env::var("KEYCLOAK_AUTH_CLIENT_ID")
                .map_err(|_| "KEYCLOAK_AUTH_CLIENT_ID must be set".to_string())?,
            backend_client_id: env::var("KEYCLOAK_BACKEND_CLIENT_ID")
                .map_err(|_| "KEYCLOAK_BACKEND_CLIENT_ID must be set".to_string())?,
            backend_client_secret: env::var("KEYCLOAK_BACKEND_CLIENT_SECRET")
                .map_err(|_| "KEYCLOAK_BACKEND_CLIENT_SECRET must be set".to_string())?,
        };

        Ok(Config {
            server,
            database,
            jwt,
            keycloak,
        })
    }
}

impl KeycloakConfig {
    pub fn realm_url(&self) -> String {
        format!("{}/realms/{}", self.url, self.realm)
    }

    pub fn token_endpoint(&self) -> String {
        format!("{}/protocol/openid-connect/token", self.realm_url())
    }

    pub fn user_endpoint(&self) -> String {
        format!("{}/admin/realms/{}/users", self.url, self.realm)
    }
}
