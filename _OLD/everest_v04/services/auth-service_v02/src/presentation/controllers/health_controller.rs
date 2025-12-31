use crate::AppState;
use crate::application::health_dto::HealthResponseDto;
use crate::application::health_service::HealthService;
use actix_web::{HttpResponse, Responder, get, web};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service Health Status", body = HealthResponseDto)
    ),
    tag = "System"
)]
#[get("/health")]
pub async fn get_health(data: web::Data<AppState>) -> impl Responder {
    let health_report = HealthService::check_health(&data).await;

    if health_report.status == "ok" {
        HttpResponse::Ok().json(health_report)
    } else {
        HttpResponse::ServiceUnavailable().json(health_report)
    }
}
