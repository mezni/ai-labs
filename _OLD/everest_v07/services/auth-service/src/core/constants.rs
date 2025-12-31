pub const APP_NAME: &str = "auth-service";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const API_PREFIX: &str = "/api/v1";

pub const DEFAULT_PORT: &str = "3000";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_LOG_LEVEL: &str = "info";

pub const MAX_DB_CONNECTIONS: u32 = 5;

/// Default page size for pagination
pub const DEFAULT_PAGE_SIZE: u64 = 20;

pub const REGISTRATION_PREFIX: &str = "REG";
pub const TOKEN_PREFIX: &str = "TKN";
pub const USER_PREFIX: &str = "USR";

pub const MAX_RESEND_COUNT: i32 = 3;
pub const RESEND_COOLDOWN_MINUTES: i64 = 15;

pub const NANOID_LENGTH: usize = 16;

// Token expiration times (in seconds)
pub const ACCESS_TOKEN_EXPIRY_SECONDS: i64 = 15 * 60; // 15 minutes
pub const REFRESH_TOKEN_EXPIRY_SECONDS: i64 = 7 * 24 * 60 * 60; // 7 days

// Verification expiration times
pub const VERIFICATION_TOKEN_EXPIRY_HOURS: i64 = 24; // 24 hours
pub const VERIFICATION_CODE_EXPIRY_MINUTES: i64 = 10; // 10 minutes

// Invitation expiration
pub const INVITATION_EXPIRY_HOURS: i64 = 72; // 72 hours (3 days)

// Password reset expiration
pub const PASSWORD_RESET_EXPIRY_HOURS: i64 = 1; // 1 hour

// Rate limiting
pub const MAX_LOGIN_ATTEMPTS: i32 = 5;
pub const LOGIN_RATE_LIMIT_WINDOW_SECONDS: i64 = 15 * 60; // 15 minutes

pub const MAX_REGISTRATION_ATTEMPTS: i32 = 3;
pub const REGISTRATION_RATE_LIMIT_WINDOW_SECONDS: i64 = 60 * 60; // 1 hour

pub const MAX_VERIFICATION_RESEND_ATTEMPTS: i32 = 5;
pub const VERIFICATION_RESEND_RATE_LIMIT_WINDOW_SECONDS: i64 = 60 * 60; // 1 hour

pub const MAX_PASSWORD_RESET_ATTEMPTS: i32 = 3;
pub const PASSWORD_RESET_RATE_LIMIT_WINDOW_SECONDS: i64 = 60 * 60; // 1 hour

// ID lengths
pub const USER_ID_LENGTH: usize = 32;
pub const REGISTRATION_ID_LENGTH: usize = 32;
pub const INVITATION_ID_LENGTH: usize = 32;
pub const TOKEN_ID_LENGTH: usize = 32;
pub const VERIFICATION_TOKEN_LENGTH: usize = 64;

// Validation constraints
pub const MIN_USERNAME_LENGTH: usize = 3;
pub const MAX_USERNAME_LENGTH: usize = 100;

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 128;

pub const MAX_EMAIL_LENGTH: usize = 255;
pub const MAX_PHONE_LENGTH: usize = 20;

pub const MAX_NAME_LENGTH: usize = 100;

// Verification code
pub const VERIFICATION_CODE_LENGTH: usize = 6;

// JWT
pub const JWT_ALGORITHM: &str = "HS256";
pub const JWT_ISSUER: &str = "auth-service";

// Rate limit actions
pub const RATE_LIMIT_LOGIN: &str = "login";
pub const RATE_LIMIT_REGISTRATION: &str = "registration";
pub const RATE_LIMIT_VERIFICATION: &str = "verification";
pub const RATE_LIMIT_PASSWORD_RESET: &str = "password_reset";

// Audit log actions
pub const AUDIT_ACTION_LOGIN: &str = "login";
pub const AUDIT_ACTION_LOGOUT: &str = "logout";
pub const AUDIT_ACTION_REGISTER: &str = "register";
pub const AUDIT_ACTION_VERIFY: &str = "verify";
pub const AUDIT_ACTION_PASSWORD_RESET: &str = "password_reset";
pub const AUDIT_ACTION_TOKEN_REFRESH: &str = "token_refresh";

// Keycloak sync actions
pub const KEYCLOAK_SYNC_CREATE: &str = "create";
pub const KEYCLOAK_SYNC_UPDATE: &str = "update";
pub const KEYCLOAK_SYNC_DELETE: &str = "delete";
pub const KEYCLOAK_SYNC_ROLE_UPDATE: &str = "role_update";
pub const KEYCLOAK_SYNC_STATUS_UPDATE: &str = "status_update";

// Default values
pub const DEFAULT_LANGUAGE: &str = "en";
pub const DEFAULT_TIMEZONE: &str = "UTC";
pub const DEFAULT_THEME: &str = "light";
pub const DEFAULT_NETWORK_ID: &str = "";
pub const DEFAULT_STATION_ID: &str = "";