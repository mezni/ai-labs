use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "operator")]
    Operator,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Admin => write!(f, "admin"),
            Role::Partner => write!(f, "partner"),
            Role::Operator => write!(f, "operator"),
        }
    }
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "deleted")]
    Deleted,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Pending => write!(f, "pending"),
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Locked => write!(f, "locked"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::Deleted => write!(f, "deleted"),
        }
    }
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Pending
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "lowercase")]
pub enum RegistrationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl std::fmt::Display for RegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationStatus::Pending => write!(f, "pending"),
            RegistrationStatus::Verified => write!(f, "verified"),
            RegistrationStatus::Expired => write!(f, "expired"),
            RegistrationStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl Default for RegistrationStatus {
    fn default() -> Self {
        RegistrationStatus::Pending
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "lowercase")]
pub enum Source {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "sso")]
    Sso,
    #[serde(rename = "internal")]
    Internal,
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Web => write!(f, "web"),
            Source::Mobile => write!(f, "mobile"),
            Source::Api => write!(f, "api"),
            Source::Admin => write!(f, "admin"),
            Source::Import => write!(f, "import"),
            Source::Sso => write!(f, "sso"),
            Source::Internal => write!(f, "internal"),
        }
    }
}

impl Default for Source {
    fn default() -> Self {
        Source::Web
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "lowercase")]
pub enum InvitationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}

impl std::fmt::Display for InvitationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvitationStatus::Pending => write!(f, "pending"),
            InvitationStatus::Accepted => write!(f, "accepted"),
            InvitationStatus::Cancelled => write!(f, "cancelled"),
            InvitationStatus::Expired => write!(f, "expired"),
        }
    }
}

impl Default for InvitationStatus {
    fn default() -> Self {
        InvitationStatus::Pending
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SyncAction {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "role_update")]
    RoleUpdate,
    #[serde(rename = "status_update")]
    StatusUpdate,
}

impl std::fmt::Display for SyncAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncAction::Create => write!(f, "create"),
            SyncAction::Update => write!(f, "update"),
            SyncAction::Delete => write!(f, "delete"),
            SyncAction::RoleUpdate => write!(f, "role_update"),
            SyncAction::StatusUpdate => write!(f, "status_update"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "skipped")]
    Skipped,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncStatus::Success => write!(f, "success"),
            SyncStatus::Failed => write!(f, "failed"),
            SyncStatus::Skipped => write!(f, "skipped"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AuditAction {
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "logout")]
    Logout,
    #[serde(rename = "register")]
    Register,
    #[serde(rename = "verify")]
    Verify,
    #[serde(rename = "password_reset")]
    PasswordReset,
    #[serde(rename = "token_refresh")]
    TokenRefresh,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::Login => write!(f, "login"),
            AuditAction::Logout => write!(f, "logout"),
            AuditAction::Register => write!(f, "register"),
            AuditAction::Verify => write!(f, "verify"),
            AuditAction::PasswordReset => write!(f, "password_reset"),
            AuditAction::TokenRefresh => write!(f, "token_refresh"),
        }
    }
}