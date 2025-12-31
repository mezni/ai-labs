// src/core/constants.rs

// User roles
pub const ROLE_USER: &str = "user";
pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_PARTNER: &str = "partner";
pub const ROLE_OPERATOR: &str = "operator";

pub const VALID_ROLES: &[&str] = &[ROLE_USER, ROLE_ADMIN, ROLE_PARTNER, ROLE_OPERATOR];

// User sources
pub const SOURCE_WEB: &str = "web";
pub const SOURCE_INTERNAL: &str = "internal";

pub const VALID_SOURCES: &[&str] = &[SOURCE_WEB, SOURCE_INTERNAL];

// ID prefixes
pub const PREFIX_USER: &str = "USR";
pub const PREFIX_NETWORK: &str = "NET";
pub const PREFIX_STATION: &str = "STA";
pub const PREFIX_REGISTRATION: &str = "REG";

// ID length (prefix + separator + nanoid)
pub const ID_LENGTH: usize = 22; // e.g., USR-xxxxxxxxxxxxxxxxx

// Validation constraints
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_EMAIL_LENGTH: usize = 255;
pub const MAX_USERNAME_LENGTH: usize = 100;
pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_PHONE_LENGTH: usize = 20;

// Rate limiting
pub const DEFAULT_RATE_LIMIT_REQUESTS: u32 = 100;
pub const DEFAULT_RATE_LIMIT_WINDOW_SECS: u64 = 3600; // 1 hour

// Cache durations
pub const JWT_CACHE_DURATION_SECS: u64 = 3600; // 1 hour
pub const USER_CACHE_DURATION_SECS: u64 = 300; // 5 minutes

// Pagination
pub const DEFAULT_PAGE_SIZE: i64 = 20;
pub const MAX_PAGE_SIZE: i64 = 100;

// Audit log actions
pub const ACTION_LOGIN: &str = "login";
pub const ACTION_LOGOUT: &str = "logout";
pub const ACTION_REGISTER: &str = "register";
pub const ACTION_PASSWORD_CHANGE: &str = "password_change";
pub const ACTION_PASSWORD_RESET: &str = "password_reset";
pub const ACTION_EMAIL_VERIFY: &str = "email_verify";
pub const ACTION_USER_CREATE: &str = "user_create";
pub const ACTION_USER_UPDATE: &str = "user_update";
pub const ACTION_USER_DELETE: &str = "user_delete";
pub const ACTION_ROLE_CHANGE: &str = "role_change";

// Sync actions
pub const SYNC_ACTION_CREATE: &str = "create";
pub const SYNC_ACTION_UPDATE: &str = "update";
pub const SYNC_ACTION_DELETE: &str = "delete";
pub const SYNC_ACTION_ROLE_UPDATE: &str = "role_update";
pub const SYNC_ACTION_STATUS_UPDATE: &str = "status_update";

// Sync statuses
pub const SYNC_STATUS_SUCCESS: &str = "success";
pub const SYNC_STATUS_FAILED: &str = "failed";
pub const SYNC_STATUS_SKIPPED: &str = "skipped";

pub fn is_valid_role(role: &str) -> bool {
    VALID_ROLES.contains(&role)
}

pub fn is_valid_source(source: &str) -> bool {
    VALID_SOURCES.contains(&source)
}

pub fn is_internal_role(role: &str) -> bool {
    matches!(role, ROLE_ADMIN | ROLE_PARTNER | ROLE_OPERATOR)
}

pub fn requires_network_id(role: &str) -> bool {
    matches!(role, ROLE_PARTNER | ROLE_OPERATOR)
}

pub fn requires_station_id(role: &str) -> bool {
    role == ROLE_OPERATOR
}
