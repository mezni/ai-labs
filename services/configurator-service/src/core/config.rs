use serde::Deserialize;
use crate::core::errors::ServiceError;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: Option<String>,
}

impl AppConfig {
    pub fn new() -> Result<Self, ServiceError> {
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| ServiceError::validation("PORT must be a valid number"))?;
        
        let log_level = std::env::var("LOG_LEVEL").ok();

        Ok(AppConfig {
            host,
            port,
            log_level,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_log_level(&self) -> String {
        self.log_level.clone().unwrap_or_else(|| "info".to_string())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            log_level: None,
        }
    }
}