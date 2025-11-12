use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub db_max_connections: u32,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost/ev_db".to_string());
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let db_max_connections = env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(crate::core::constants::DEFAULT_MAX_CONNECTIONS);

        Config {
            server_host,
            server_port,
            log_level,
            database_url,
            jwt_secret,
            db_max_connections,
        }
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    pub fn api_base_url(&self) -> String {
        format!("http://{}{}", self.server_address(), crate::core::constants::API_PREFIX)
    }
}