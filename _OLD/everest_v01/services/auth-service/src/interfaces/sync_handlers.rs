// src/interfaces/sync_handlers.rs
use crate::core::{AppError, extract_claims};
use crate::jobs::KeycloakSyncJob;
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SyncStatsResponse {
    pub total: i64,
    pub success: i64,
    pub failed: i64,
    pub skipped: i64,
    pub last_sync_at: Option<chrono::DateTime<chrono::Utc>>,
    pub success_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TriggerSyncResponse {
    pub message: String,
    pub synced: usize,
    pub failed: usize,
    pub skipped: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SyncUserResponse {
    pub message: String,
    pub user_id: String,
}

/// Get Keycloak sync statistics
#[utoipa::path(
    get,
    path = "/api/v1/admin/keycloak-sync/stats",
    tag = "Admin - Sync",
    responses(
        (status = 200, description = "Sync statistics retrieved", body = SyncStatsResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/stats")]
pub async fn get_sync_stats(
    sync_job: web::Data<KeycloakSyncJob>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can view sync statistics".to_string(),
        ));
    }

    let stats = sync_job.get_sync_stats().await?;

    let success_rate = if stats.total > 0 {
        (stats.success as f64 / stats.total as f64) * 100.0
    } else {
        0.0
    };

    let response = SyncStatsResponse {
        total: stats.total,
        success: stats.success,
        failed: stats.failed,
        skipped: stats.skipped,
        last_sync_at: stats.last_sync_at,
        success_rate,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Manually trigger Keycloak sync
#[utoipa::path(
    post,
    path = "/api/v1/admin/keycloak-sync/trigger",
    tag = "Admin - Sync",
    responses(
        (status = 200, description = "Sync triggered successfully", body = TriggerSyncResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/trigger")]
pub async fn trigger_sync(
    sync_job: web::Data<KeycloakSyncJob>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can trigger sync".to_string(),
        ));
    }

    let result = sync_job.trigger_sync().await?;

    Ok(HttpResponse::Ok().json(TriggerSyncResponse {
        message: "Keycloak sync completed".to_string(),
        synced: result.synced,
        failed: result.failed,
        skipped: result.skipped,
    }))
}

/// Sync a specific user
#[utoipa::path(
    post,
    path = "/api/v1/admin/keycloak-sync/users/{user_id}",
    tag = "Admin - Sync",
    params(
        ("user_id" = String, Path, description = "User ID to sync")
    ),
    responses(
        (status = 200, description = "User synced successfully", body = SyncUserResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/users/{user_id}")]
pub async fn sync_user(
    sync_job: web::Data<KeycloakSyncJob>,
    user_id: web::Path<String>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can sync users".to_string(),
        ));
    }

    let user_id = user_id.into_inner();
    sync_job.sync_user_by_id(&user_id).await?;

    Ok(HttpResponse::Ok().json(SyncUserResponse {
        message: "User synced successfully".to_string(),
        user_id,
    }))
}
