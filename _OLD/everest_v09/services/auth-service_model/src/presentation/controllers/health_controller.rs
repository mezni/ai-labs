use crate::application::health_service::HealthService;
use actix_web::{HttpResponse, get, web};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy")
    ),
    tag = "Health"
)]
#[get("/health")]
pub async fn health_check(service: web::Data<Arc<HealthService>>) -> HttpResponse {
    match service.check().await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(_) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "unhealthy"
        })),
    }
}
