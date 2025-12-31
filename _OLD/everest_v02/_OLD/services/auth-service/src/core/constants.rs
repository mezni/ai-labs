pub const REGISTRATION_EXPIRY_HOURS: i64 = 24;
pub const REGISTRATION_ID_PREFIX: &str = "REG-";
pub const USER_ID_PREFIX: &str = "USR-";
pub const NANOID_LENGTH: usize = 18;

// Rate limiting
pub const REGISTRATION_RATE_LIMIT: i32 = 5; // per hour per IP
pub const RATE_LIMIT_WINDOW_MINUTES: i64 = 60;
