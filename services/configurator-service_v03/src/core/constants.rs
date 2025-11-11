// Application constants

// Database constants
pub mod db {
    pub const DEFAULT_PAGE_SIZE: u32 = 20;
    pub const MAX_PAGE_SIZE: u32 = 100;
    pub const CONNECTION_TIMEOUT: u64 = 30; // seconds
    pub const IDLE_TIMEOUT: u64 = 300; // seconds
    pub const MAX_LIFETIME: u64 = 1800; // seconds
}

// Network constants
pub mod network {
    pub const NAME_MAX_LENGTH: usize = 255;
    pub const OWNER_NAME_MAX_LENGTH: usize = 255;
    pub const TYPE_INDIVIDUAL: &str = "individual";
    pub const TYPE_COMPANY: &str = "company";
}

// Validation constants
pub mod validation {
    pub const EMAIL_MIN_LENGTH: usize = 3;
    pub const EMAIL_MAX_LENGTH: usize = 255;
    pub const PHONE_MIN_LENGTH: usize = 6;
    pub const PHONE_MAX_LENGTH: usize = 50;
    pub const ADDRESS_MIN_LENGTH: usize = 5;
    pub const ADDRESS_MAX_LENGTH: usize = 500;
}

// API constants
pub mod api {
    pub const V1_PREFIX: &str = "/api/v1";
    pub const HEALTH_CHECK_PATH: &str = "/health";
    pub const METRICS_PATH: &str = "/metrics";
}

// Pagination constants
pub mod pagination {
    pub const DEFAULT_PAGE: u32 = 1;
    pub const DEFAULT_LIMIT: u32 = 20;
}

// Cache constants
pub mod cache {
    pub const TTL: u64 = 300; // 5 minutes
}

// Date/Time formats
pub mod datetime {
    pub const DATE_FORMAT: &str = "%Y-%m-%d";
    pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    pub const TIMESTAMP_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
}

// Error messages
pub mod error {
    pub const DATABASE_CONNECTION: &str = "Database connection failed";
    pub const VALIDATION_FAILED: &str = "Validation failed";
    pub const NOT_FOUND: &str = "Resource not found";
    pub const UNAUTHORIZED: &str = "Unauthorized access";
    pub const FORBIDDEN: &str = "Access forbidden";
}

// Success messages
pub mod success {
    pub const CREATED: &str = "Resource created successfully";
    pub const UPDATED: &str = "Resource updated successfully";
    pub const DELETED: &str = "Resource deleted successfully";
}

// Logging constants
pub mod log {
    pub const TARGET_API: &str = "network_api";
    pub const TARGET_DB: &str = "network_db";
    pub const TARGET_DOMAIN: &str = "network_domain";
}
