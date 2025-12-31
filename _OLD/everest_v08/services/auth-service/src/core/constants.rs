pub const APP_NAME: &str = "auth-service";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const API_PREFIX: &str = "/api/v1";

pub const DEFAULT_PORT: &str = "3000";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_LOG_LEVEL: &str = "info";

pub const MIN_DB_CONNECTIONS: u32 = 2;
pub const MAX_DB_CONNECTIONS: u32 = 10;
