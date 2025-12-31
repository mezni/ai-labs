use crate::AppState;
use crate::application::dtos::health::HealthResponse;
use crate::application::health_service::HealthService; // Import the service
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
    // 1. Get the Arc<AppState> from web::Data
    let app_state_arc = state.into_inner();

    // 2. Pass the Arc to the service
    let svc = HealthService::new(app_state_arc);

    match svc.check().await {
        Ok(report) => {
            let response = HealthResponse {
                status: report.status,
                database: report.database,
            };

            if response.database {
                HttpResponse::Ok().json(response)
            } else {
                HttpResponse::ServiceUnavailable().json(response)
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
