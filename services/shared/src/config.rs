use crate::constants;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub environment: String,
    pub service_name: String,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub auth: AuthConfig,
    pub features: HashMap<String, bool>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            environment: constants::ENV_DEVELOPMENT.to_string(),
            service_name: constants::SERVICE_NAME.to_string(),
            database: DatabaseConfig {
                url: String::new(),
                max_connections: constants::MAX_DB_CONNECTIONS,
                timeout_seconds: constants::DB_TIMEOUT_SECONDS,
            },
            server: ServerConfig {
                port: constants::DEFAULT_HTTP_PORT,
                host: "0.0.0.0".to_string(),
                cors_origins: vec!["*".to_string()],
            },
            logging: LoggingConfig {
                level: constants::DEFAULT_LOG_LEVEL.to_string(),
                format: "text".to_string(),
            },
            auth: AuthConfig {
                jwt_secret: String::new(),
                jwt_expiry_hours: 24,
            },
            features: HashMap::new(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let environment = env::var(constants::ENVIRONMENT_ENV)
            .unwrap_or_else(|_| constants::ENV_DEVELOPMENT.to_string());

        let features = Self::parse_feature_flags();

        Ok(Self {
            environment,
            service_name: env::var("SERVICE_NAME")
                .map_err(|_| ConfigError::MissingVariable("SERVICE_NAME".to_string()))?,
            database: DatabaseConfig {
                url: env::var(constants::DATABASE_URL_ENV).unwrap_or_default(),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| constants::MAX_DB_CONNECTIONS.to_string())
                    .parse()
                    .unwrap_or(constants::MAX_DB_CONNECTIONS),
                timeout_seconds: env::var("DATABASE_TIMEOUT_SECONDS")
                    .unwrap_or_else(|_| constants::DB_TIMEOUT_SECONDS.to_string())
                    .parse()
                    .unwrap_or(constants::DB_TIMEOUT_SECONDS),
            },
            server: ServerConfig {
                port: env::var(constants::HTTP_PORT_ENV)
                    .map_err(|_| ConfigError::MissingVariable("PORT".to_string()))?
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue("PORT must be a number".to_string()))?,
                host: env::var("HOST")
                    .map_err(|_| ConfigError::MissingVariable("HOST".to_string()))?,
                cors_origins: env::var(constants::CORS_ORIGINS_ENV)
                    .unwrap_or_else(|_| "*".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            logging: LoggingConfig {
                level: env::var(constants::LOG_LEVEL_ENV)
                    .unwrap_or_else(|_| constants::DEFAULT_LOG_LEVEL.to_string()),
                format: env::var("LOG_FORMAT").unwrap_or_else(|_| "text".to_string()),
            },
            auth: AuthConfig {
                jwt_secret: env::var(constants::JWT_SECRET_ENV).unwrap_or_default(),
                jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                    .unwrap_or_else(|_| "24".to_string())
                    .parse()
                    .unwrap_or(24),
            },
            features,
        })
    }

    fn parse_feature_flags() -> HashMap<String, bool> {
        env::var("FEATURE_FLAGS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|flag| {
                let parts: Vec<&str> = flag.split('=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1] == "true" || parts[1] == "1"))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn is_development(&self) -> bool {
        self.environment == constants::ENV_DEVELOPMENT
    }

    pub fn is_production(&self) -> bool {
        self.environment == constants::ENV_PRODUCTION
    }

    pub fn feature_enabled(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVariable(String),

    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
}
