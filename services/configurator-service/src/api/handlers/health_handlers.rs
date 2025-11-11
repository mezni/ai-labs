use crate::core::Database;
use actix_web::{HttpResponse, get, web};
use chrono::Utc;
use serde::Serialize;
use utoipa::ToSchema;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy")
    )
)]
#[get("")] // Empty path since it's under /api/v1/health scope
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/health/database",
    tag = "Health",
    responses(
        (status = 200, description = "Database is healthy", body = HealthResponse),
        (status = 503, description = "Database is unhealthy")
    )
)]
#[get("/database")] // Relative to /api/v1/health scope
pub async fn database_health(db: web::Data<Database>) -> HttpResponse {
    let healthy = db.health_check().await;

    if healthy {
        HttpResponse::Ok().json(HealthResponse {
            status: "healthy".to_string(),
            timestamp: Utc::now().to_rfc3339(),
        })
    } else {
        HttpResponse::ServiceUnavailable().json(HealthResponse {
            status: "unhealthy".to_string(),
            timestamp: Utc::now().to_rfc3339(),
        })
    }
}

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}
