use crate::core::errors::AppError;
use serde::Deserialize;
use std::env;

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
}

impl AppConfig {
    pub fn new() -> Result<Self, AppError> {
        dotenvy::dotenv().ok(); // Load .env if present

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| AppError::validation("PORT must be a valid number"))?;

        let log_level = env::var("LOG_LEVEL").ok();
        let database_url = env::var("DATABASE_URL").ok();

        let db_host = env::var("DB_HOST").ok();
        let db_port = env::var("DB_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok());
        let db_user = env::var("DB_USER").ok();
        let db_password = env::var("DB_PASSWORD").ok();
        let db_name = env::var("DB_NAME").ok();

        Ok(AppConfig {
            host,
            port,
            log_level,
            database_url,
            db_host,
            db_port,
            db_user,
            db_password,
            db_name,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_log_level(&self) -> String {
        self.log_level.clone().unwrap_or_else(|| "info".to_string())
    }

    /// Build a PostgreSQL connection string (useful for deadpool_postgres)
    pub fn get_database_url(&self) -> Result<String, AppError> {
        if let Some(url) = &self.database_url {
            return Ok(url.clone());
        }

        let host = self
            .db_host
            .clone()
            .ok_or_else(|| AppError::validation("DB_HOST not set"))?;
        let port = self
            .db_port
            .ok_or_else(|| AppError::validation("DB_PORT not set"))?;
        let user = self
            .db_user
            .clone()
            .ok_or_else(|| AppError::validation("DB_USER not set"))?;
        let password = self
            .db_password
            .clone()
            .ok_or_else(|| AppError::validation("DB_PASSWORD not set"))?;
        let dbname = self
            .db_name
            .clone()
            .ok_or_else(|| AppError::validation("DB_NAME not set"))?;

        Ok(format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, dbname
        ))
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
        }
    }
}
