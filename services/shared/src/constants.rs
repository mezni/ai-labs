// Database constants
pub const DATABASE_URL_ENV: &str = "DATABASE_URL";
pub const MAX_DB_CONNECTIONS: u32 = 20;
pub const DB_TIMEOUT_SECONDS: u64 = 30;

// Application constants
pub const SERVICE_NAME: &str = "networks-service";
pub const API_VERSION: &str = "v1";
pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;

// Network type constants
pub const NETWORK_TYPE_INDIVIDUAL: &str = "individual";
pub const NETWORK_TYPE_COMPANY: &str = "company";

// Validation constants
pub const MAX_NAME_LENGTH: usize = 255;
pub const MAX_EMAIL_LENGTH: usize = 255;
pub const MAX_PHONE_LENGTH: usize = 50;

// HTTP constants
pub const DEFAULT_HTTP_PORT: u16 = 8080;
pub const HTTP_PORT_ENV: &str = "PORT";
pub const HEALTH_CHECK_PATH: &str = "/health";
pub const API_DOCS_PATH: &str = "/api-docs";

// CORS constants
pub const CORS_ORIGINS_ENV: &str = "CORS_ORIGINS";

// Logging constants
pub const LOG_LEVEL_ENV: &str = "LOG_LEVEL";
pub const DEFAULT_LOG_LEVEL: &str = "info";

// Environment constants
pub const ENV_PRODUCTION: &str = "production";
pub const ENV_STAGING: &str = "staging";
pub const ENV_DEVELOPMENT: &str = "development";
pub const ENV_TEST: &str = "test";
pub const ENVIRONMENT_ENV: &str = "ENVIRONMENT";

// JWT constants
pub const JWT_SECRET_ENV: &str = "JWT_SECRET";
pub const JWT_EXPIRATION_HOURS: i64 = 24;
