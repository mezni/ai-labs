// ID Prefixes
pub const USER_PREFIX: &str = "USR";
pub const REGISTRATION_PREFIX: &str = "REG";
pub const TOKEN_PREFIX: &str = "TKN";

// ID Length
pub const NANOID_LENGTH: usize = 16;

// Verification
pub const MAX_RESEND_COUNT: i32 = 3;
pub const RESEND_COOLDOWN_MINUTES: i64 = 5;

// Password validation
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 128;