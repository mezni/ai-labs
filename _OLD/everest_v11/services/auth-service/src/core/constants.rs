// Verification settings
pub const VERIFICATION_EXPIRY_HOURS: i64 = 24;
pub const MAX_RESEND_ATTEMPTS: i32 = 3;
pub const RESEND_COOLDOWN_MINUTES: i64 = 5;

// Invitation settings
pub const INVITATION_EXPIRY_DAYS: i64 = 7;
pub const INVITATION_CODE_LENGTH: usize = 12;

// Token settings
pub const TOKEN_EXPIRY_SECONDS: i64 = 3600; // 1 hour
pub const REFRESH_TOKEN_EXPIRY_DAYS: i64 = 30;

// Role constants
pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_USER: &str = "user";

// Status constants
pub const STATUS_ACTIVE: &str = "active";
pub const STATUS_INACTIVE: &str = "inactive";
pub const STATUS_PENDING_VERIFICATION: &str = "pending_verification";
pub const STATUS_VERIFICATION_EXPIRED: &str = "verification_expired";
pub const STATUS_SUSPENDED: &str = "suspended";

// Invitation status
pub const INVITATION_STATUS_PENDING: &str = "pending";
pub const INVITATION_STATUS_ACCEPTED: &str = "accepted";
pub const INVITATION_STATUS_EXPIRED: &str = "expired";
pub const INVITATION_STATUS_CANCELLED: &str = "cancelled";