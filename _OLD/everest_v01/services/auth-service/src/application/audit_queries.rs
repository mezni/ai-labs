// src/application/audit_queries.rs
use crate::application::audit_dtos::*;
use crate::core::errors::AppResult;
use crate::domain::repositories::{AuditLogFilters, AuditLogRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuditQueries {
    audit_repo: Arc<dyn AuditLogRepository>,
}

impl AuditQueries {
    pub fn new(audit_repo: Arc<dyn AuditLogRepository>) -> Self {
        Self { audit_repo }
    }

    /// Get login audit logs with pagination
    pub async fn get_login_logs(
        &self,
        request: GetAuditLogsRequest,
    ) -> AppResult<PaginatedAuditLogsResponse> {
        let filters = AuditLogFilters {
            user_id: request.user_id,
            action: request.action,
            from_date: request.from_date,
            to_date: request.to_date,
            success: request.success,
            page: Some(request.page),
            page_size: Some(request.page_size),
        };

        let logs = self.audit_repo.get_login_logs(filters.clone()).await?;
        let total = self.audit_repo.count_login_logs(filters).await?;

        let total_pages = (total as f64 / request.page_size as f64).ceil() as i64;

        Ok(PaginatedAuditLogsResponse {
            logs: logs.into_iter().map(LoginAuditLogResponse::from).collect(),
            total,
            page: request.page,
            page_size: request.page_size,
            total_pages,
        })
    }
}
