use crate::application::health::{HealthResponse, VersionResponse};
use actix_web::{HttpResponse, Responder, get};

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Health & Status",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
#[get("/health")]
pub async fn health_check() -> impl Responder {
    let response = HealthResponse {
        status: "ok".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    HttpResponse::Ok().json(response)
}

/// Prometheus metrics endpoint
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    tag = "Health & Status",
    responses(
        (status = 200, description = "Prometheus metrics", content_type = "text/plain")
    )
)]
#[get("/metrics")]
pub async fn metrics() -> impl Responder {
    // TODO: Implement actual Prometheus metrics
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("# Metrics placeholder\n")
}

/// API version information
#[utoipa::path(
    get,
    path = "/api/v1/version",
    tag = "Health & Status",
    responses(
        (status = 200, description = "API version information", body = VersionResponse)
    )
)]
#[get("/version")]
pub async fn version() -> impl Responder {
    let response = VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        build_date: "2024-01-01".to_string(), // TODO: Set during build
        git_commit: "dev".to_string(),        // TODO: Set during build
    };
    HttpResponse::Ok().json(response)
}
