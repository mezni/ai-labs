use crate::AppState;
use crate::application::health_service::{HealthReport, HealthService};
use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "System",
    responses(
        (status = 200, description = "System report generated", body = HealthReport),
        (status = 503, description = "System is degraded", body = HealthReport)
    )
)]
#[get("/health")]
pub async fn get_health(state: web::Data<AppState>) -> impl Responder {
    let report = HealthService::check_system_status(&state.db).await;

    if report.db_connected {
        HttpResponse::Ok().json(report)
    } else {
        HttpResponse::ServiceUnavailable().json(report)
    }
}
