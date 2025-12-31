use serde::Deserialize;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_issuer: String,
    pub jwks_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/locate_db".to_string()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid u16"),
            jwt_issuer: env::var("JWT_ISSUER")
                .unwrap_or_else(|_| "http://localhost:5080/realms/myrealm".to_string()),
            jwks_url: env::var("JWKS_URL").unwrap_or_else(|_| {
                "http://localhost:5080/realms/myrealm/protocol/openid-connect/certs".to_string()
            }),
        })
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}
