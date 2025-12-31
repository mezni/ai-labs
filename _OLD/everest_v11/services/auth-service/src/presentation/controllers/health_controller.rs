use actix_web::{get, web, HttpResponse};

use crate::{
    application::dtos::health::HealthResponse,
    AppState,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
#[get("/health")]
async fn health_check(state: web::Data<AppState>) -> HttpResponse {
    match state.health_service.check_health().await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::ServiceUnavailable().json(HealthResponse {
            status: "unhealthy".to_string(),
            database: "disconnected".to_string(),
            timestamp: chrono::Utc::now(),
        }),
    }
}