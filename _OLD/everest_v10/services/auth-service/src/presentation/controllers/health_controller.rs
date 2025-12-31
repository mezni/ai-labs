use actix_web::{HttpResponse, Responder, get, web};

use crate::AppState;
use crate::application::dtos::health::{HealthResponse, HealthStatus};
use crate::application::health_service::HealthService;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service Health Status", body = HealthResponse),
        (status = 503, description = "Service Unhealthy", body = HealthResponse)
    ),
    tag = "Health"
)]
#[get("/health")]
pub async fn health_check(state: web::Data<AppState>) -> impl Responder {
    let app_state = state.into_inner();
    let svc = HealthService::new(app_state);

    match svc.check().await {
        Ok((status, details)) => {
            let response = HealthResponse {
                status: status.clone(),
                details: Some(details),
            };

            match status {
                HealthStatus::Up => HttpResponse::Ok().json(response),
                HealthStatus::Down => HttpResponse::ServiceUnavailable().json(response),
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
