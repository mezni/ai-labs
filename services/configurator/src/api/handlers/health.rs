use actix_web::HttpResponse;
use crate::application::services::networks::NetworkService;
use tracing::error;
use utoipa::path;

// Health check handler (public)
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy"),
        (status = 503, description = "Service is unhealthy")
    )
)]
pub async fn health_check(service: actix_web::web::Data<NetworkService>) -> HttpResponse {
    match service.health_check().await {
        Ok(_) => HttpResponse::Ok().json("Service is healthy"),
        Err(e) => {
            error!("Health check failed: {}", e);
            HttpResponse::ServiceUnavailable().json("Service is unhealthy")
        }
    }
}

// Pool stats handler (public)
#[utoipa::path(
    get,
    path = "/api/v1/health/pool",
    responses(
        (status = 200, description = "Pool statistics", body = crate::core::database::PoolStats),
        (status = 503, description = "Service is unhealthy")
    )
)]
pub async fn pool_stats(service: actix_web::web::Data<NetworkService>) -> HttpResponse {
    match service.get_pool_stats().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => {
            error!("Failed to get pool stats: {}", e);
            HttpResponse::ServiceUnavailable().json("Service is unhealthy")
        }
    }
}