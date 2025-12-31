pub const APP_NAME: &str = "auth-service";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const API_PREFIX: &str = "/api/v1";

// Server defaults
pub const DEFAULT_PORT: &str = "3000";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_LOG_LEVEL: &str = "info";

// Database
pub const MAX_DB_CONNECTIONS: u32 = 10;
pub const MIN_DB_CONNECTIONS: u32 = 2;

// IDs / Tokens
pub const NANOID_LENGTH: usize = 16;

// Verification
pub const DEFAULT_VERIFICATION_EXPIRY_HOURS: u64 = 24;
pub const DEFAULT_REFRESH_TOKEN_EXPIRY_DAYS: u64 = 365;

pub const DEFAULT_INVITATION_EXPIRES_HOURS: u64 = 72;
