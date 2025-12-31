use crate::AppState;
use crate::application::dtos::health::HealthResponse;
use crate::application::health_service::HealthService;
use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service Health Status", body = HealthResponse),
        (status = 503, description = "Service Unhealthy", body = HealthResponse)
    ),
    tag = "System"
)]
#[get("/health")]
pub async fn health_check(state: web::Data<AppState>) -> impl Responder {
    // web::Data<T> is essentially an Arc<T>.
    // .into_inner() gives you the Arc<AppState>.
    let app_state = state.into_inner();

    // Pass the Arc to your service
    let svc = HealthService::new(app_state);

    match svc.check().await {
        Ok(report) => {
            let response = HealthResponse {
                status: report.status,
                database: report.database,
            };

            if response.database {
                HttpResponse::Ok().json(response)
            } else {
                // If DB is down, return 503
                HttpResponse::ServiceUnavailable().json(response)
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
