// src/interfaces/audit_handlers.rs
use crate::application::{AuditQueries, GetAuditLogsRequest, PaginatedAuditLogsResponse};
use crate::core::{AppError, extract_claims};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use validator::Validate;

/// Get login audit logs (admin only)
#[utoipa::path(
    get,
    path = "/api/v1/admin/login-audit",
    tag = "Admin - Audit",
    params(
        ("user_id" = Option<String>, Query, description = "Filter by user ID"),
        ("action" = Option<String>, Query, description = "Filter by action"),
        ("success" = Option<bool>, Query, description = "Filter by success status"),
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("page_size" = Option<i64>, Query, description = "Page size (default: 20)")
    ),
    responses(
        (status = 200, description = "Audit logs retrieved", body = PaginatedAuditLogsResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("")]
pub async fn get_login_audit_logs(
    audit_queries: web::Data<AuditQueries>,
    query: web::Query<GetAuditLogsRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can view audit logs".to_string(),
        ));
    }

    query
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let response = audit_queries.get_login_logs(query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(response))
}
