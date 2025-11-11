use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub database_url: String,
    pub cors_allowed_origins: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".into()).parse().unwrap_or(8080);
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into());
        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "*".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".into());
            let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".into());
            let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".into());
            let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "password".into());
            let db_name = env::var("DB_NAME").unwrap_or_else(|_| "ev_db".into());

            format!("postgres://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name)
        });

        Self {
            host,
            port,
            log_level,
            database_url,
            cors_allowed_origins,
        }
    }
}
