use crate::constants;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

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
            environment: constants::ENV_DEVELOPMENT.into(),
            service_name: constants::SERVICE_NAME.into(),
            database: DatabaseConfig {
                url: String::new(),
                max_connections: constants::MAX_DB_CONNECTIONS,
                timeout_seconds: constants::DB_TIMEOUT_SECONDS,
            },
            server: ServerConfig {
                port: constants::DEFAULT_HTTP_PORT,
                host: "0.0.0.0".into(),
                cors_origins: vec!["*".into()],
            },
            logging: LoggingConfig {
                level: constants::DEFAULT_LOG_LEVEL.into(),
                format: "text".into(),
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
            .unwrap_or_else(|_| constants::ENV_DEVELOPMENT.into());

        let features = Self::parse_feature_flags();

        Ok(Self {
            environment,
            service_name: get_env("SERVICE_NAME")?,
            database: DatabaseConfig {
                url: env::var(constants::DATABASE_URL_ENV).unwrap_or_default(),
                max_connections: get_env_parse("DATABASE_MAX_CONNECTIONS", constants::MAX_DB_CONNECTIONS),
                timeout_seconds: get_env_parse("DATABASE_TIMEOUT_SECONDS", constants::DB_TIMEOUT_SECONDS),
            },
            server: ServerConfig {
                port: get_env_parse("PORT", constants::DEFAULT_HTTP_PORT),
                host: get_env("HOST")?,
                cors_origins: env::var(constants::CORS_ORIGINS_ENV)
                    .unwrap_or_else(|_| "*".into())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            logging: LoggingConfig {
                level: env::var(constants::LOG_LEVEL_ENV)
                    .unwrap_or_else(|_| constants::DEFAULT_LOG_LEVEL.into()),
                format: env::var("LOG_FORMAT").unwrap_or_else(|_| "text".into()),
            },
            auth: AuthConfig {
                jwt_secret: env::var(constants::JWT_SECRET_ENV).unwrap_or_default(),
                jwt_expiry_hours: get_env_parse("JWT_EXPIRY_HOURS", 24),
            },
            features,
        })
    }

    fn parse_feature_flags() -> HashMap<String, bool> {
        env::var("FEATURE_FLAGS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|flag| {
                let parts: Vec<_> = flag.split('=').collect();
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

fn get_env(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::MissingVariable(key.into()))
}

fn get_env_parse<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr + Copy,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVariable(String),
}
