use crate::core::errors::AppError;
use serde::Deserialize;
use std::env;

/// Application configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: Option<String>,
    pub database_url: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<u16>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
    pub db_name: Option<String>,
    pub max_db_connections: Option<u32>,
}

impl AppConfig {
    /// Creates a new AppConfig instance from environment variables
    pub fn new() -> Result<Self, AppError> {
        dotenvy::dotenv().ok(); // Load .env if present

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = Self::parse_port(env::var("PORT").ok())?;
        let log_level = env::var("LOG_LEVEL").ok();
        let database_url = env::var("DATABASE_URL").ok();

        let db_config = Self::parse_db_config()?;
        let max_db_connections = Self::parse_max_connections(env::var("MAX_DB_CONNECTIONS").ok())?;

        Ok(AppConfig {
            host,
            port,
            log_level,
            database_url,
            db_host: db_config.host,
            db_port: db_config.port,
            db_user: db_config.user,
            db_password: db_config.password,
            db_name: db_config.name,
            max_db_connections,
        })
    }

    /// Returns the server address in the format "host:port"
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Returns the log level, defaulting to "info" if not set
    pub fn get_log_level(&self) -> String {
        self.log_level.clone().unwrap_or_else(|| "info".to_string())
    }

    /// Returns the maximum number of database connections, defaulting to 20
    pub fn get_max_db_connections(&self) -> u32 {
        self.max_db_connections.unwrap_or(20)
    }

    /// Builds a PostgreSQL connection string
    pub fn get_database_url(&self) -> Result<String, AppError> {
        if let Some(url) = &self.database_url {
            return Ok(url.clone());
        }

        let host = self
            .db_host
            .as_ref()
            .ok_or_else(|| AppError::validation("DB_HOST not set"))?;
        let port = self
            .db_port
            .ok_or_else(|| AppError::validation("DB_PORT not set"))?;
        let user = self
            .db_user
            .as_ref()
            .ok_or_else(|| AppError::validation("DB_USER not set"))?;
        let password = self
            .db_password
            .as_ref()
            .ok_or_else(|| AppError::validation("DB_PASSWORD not set"))?;
        let dbname = self
            .db_name
            .as_ref()
            .ok_or_else(|| AppError::validation("DB_NAME not set"))?;

        Ok(format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, dbname
        ))
    }

    // Helper functions
    fn parse_port(port: Option<String>) -> Result<u16, AppError> {
        port.unwrap_or_else(|| "8080".to_string())
            .parse()
            .map_err(|_| AppError::validation("PORT must be a valid number"))
    }

    fn parse_max_connections(connections: Option<String>) -> Result<Option<u32>, AppError> {
        connections
            .map(|c| {
                c.parse()
                    .map_err(|_| AppError::validation("MAX_DB_CONNECTIONS must be a valid number"))
            })
            .transpose()
    }

    fn parse_db_config() -> Result<DbConfig, AppError> {
        Ok(DbConfig {
            host: env::var("DB_HOST").ok(),
            port: env::var("DB_PORT").ok().and_then(|p| p.parse::<u16>().ok()),
            user: env::var("DB_USER").ok(),
            password: env::var("DB_PASSWORD").ok(),
            name: env::var("DB_NAME").ok(),
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            log_level: None,
            database_url: None,
            db_host: None,
            db_port: None,
            db_user: None,
            db_password: None,
            db_name: None,
            max_db_connections: None,
        }
    }
}

// Helper struct for database configuration
struct DbConfig {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    name: Option<String>,
}
