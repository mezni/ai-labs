pub const APP_NAME: &str = "auth-service";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const API_PREFIX: &str = "/api/v1";

pub const DEFAULT_PORT: &str = "3000";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_LOG_LEVEL: &str = "info";

pub const MAX_DB_CONNECTIONS: u32 = 5;

/// Registration expiration time in hours
pub const REGISTRATION_EXPIRATION_HOURS: i64 = 24;

/// Default page size for pagination
pub const DEFAULT_PAGE_SIZE: u64 = 20;

/// Maximum attempts for verification before lockout (example)
pub const MAX_VERIFICATION_ATTEMPTS: u8 = 5;
