use crate::core::Database;
use actix_web::{HttpResponse, get, web};
use chrono::Utc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(health_check, database_health),
    tags(
        (name = "Health", description = "Health check endpoints")
    )
)]
pub struct HealthApiDoc;

// --- Health check ---
#[utoipa::path(
    get,
    path = "/api/v1/health", // full path as exposed by Actix
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy")
    )
)]
#[get("")] // within Actix scope("/health")
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/health/database", // full path as exposed by Actix
    tag = "Health",
    responses(
        (status = 200, description = "Database is healthy", body = HealthResponse),
        (status = 503, description = "Database is unhealthy")
    )
)]
#[get("/database")]
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

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}
