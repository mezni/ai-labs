use actix_web::{get, web, HttpResponse};
use std::sync::Arc;
use tracing::{info, error};

use crate::application::{health_service::HealthService, dtos::health_dto::HealthResponse};

/// Health check endpoint
///
/// Returns the health status of the service and its dependencies.
/// - `healthy`: All services are operational
/// - `degraded`: Core services are operational but some dependencies are down
/// - `unhealthy`: Core services are not operational
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy or degraded", body = HealthResponse),
        (status = 503, description = "Service is unhealthy", body = HealthResponse),
    )
)]
#[get("/health")]
pub async fn health(
    service: web::Data<Arc<HealthService>>,
) -> actix_web::Result<HttpResponse> {
    info!("Health check request received");
    
    match service.check_health().await {
        Ok(response) => {
            // Return 503 if unhealthy, 200 otherwise
            let status_code = match response.status {
                crate::application::dtos::health_dto::HealthStatus::Unhealthy => {
                    error!("Health check failed: service is unhealthy");
                    actix_web::http::StatusCode::SERVICE_UNAVAILABLE
                }
                crate::application::dtos::health_dto::HealthStatus::Degraded => {
                    info!("Health check: service is degraded");
                    actix_web::http::StatusCode::OK
                }
                crate::application::dtos::health_dto::HealthStatus::Healthy => {
                    info!("Health check: service is healthy");
                    actix_web::http::StatusCode::OK
                }
            };
            
            Ok(HttpResponse::build(status_code).json(response))
        }
        Err(e) => {
            error!("Health check error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Health check failed",
                "message": e.to_string()
            })))
        }
    }
}

/// Readiness probe endpoint
///
/// Used by orchestration systems (Kubernetes, Docker) to determine
/// if the service is ready to accept traffic.
#[utoipa::path(
    get,
    path = "/api/v1/health/ready",
    tag = "Health",
    responses(
        (status = 200, description = "Service is ready"),
        (status = 503, description = "Service is not ready"),
    )
)]
#[get("/health/ready")]
pub async fn readiness(
    service: web::Data<Arc<HealthService>>,
) -> actix_web::Result<HttpResponse> {
    info!("Readiness probe check");
    
    match service.check_health().await {
        Ok(response) => {
            match response.status {
                crate::application::dto::HealthStatus::Unhealthy => {
                    Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                        "ready": false,
                        "status": "unhealthy"
                    })))
                }
                _ => {
                    Ok(HttpResponse::Ok().json(serde_json::json!({
                        "ready": true,
                        "status": "ready"
                    })))
                }
            }
        }
        Err(_) => {
            Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "ready": false,
                "status": "error"
            })))
        }
    }
}

/// Liveness probe endpoint
///
/// Used by orchestration systems to determine if the service is alive.
/// This is a simple check that always returns 200 if the service is running.
#[utoipa::path(
    get,
    path = "/api/v1/health/live",
    tag = "Health",
    responses(
        (status = 200, description = "Service is alive"),
    )
)]
#[get("/health/live")]
pub async fn liveness() -> actix_web::Result<HttpResponse> {
    info!("Liveness probe check");
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "alive": true,
        "timestamp": chrono::Utc::now()
    })))
}

