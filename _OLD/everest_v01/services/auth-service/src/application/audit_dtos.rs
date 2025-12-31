// src/application/audit_dtos.rs
use crate::domain::repositories::LoginAuditLog;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Login audit log response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginAuditLogResponse {
    pub log_id: i64,
    pub user_id: String,
    pub keycloak_id: String,
    pub action: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<LoginAuditLog> for LoginAuditLogResponse {
    fn from(log: LoginAuditLog) -> Self {
        Self {
            log_id: log.log_id,
            user_id: log.user_id,
            keycloak_id: log.keycloak_id,
            action: log.action,
            ip_address: log.ip_address,
            user_agent: log.user_agent,
            success: log.success,
            error_message: log.error_message,
            created_at: log.created_at,
        }
    }
}

/// Get audit logs request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct GetAuditLogsRequest {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub success: Option<bool>,

    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: i64,

    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    20
}

/// Paginated audit logs response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedAuditLogsResponse {
    pub logs: Vec<LoginAuditLogResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}
