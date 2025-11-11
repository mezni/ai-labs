use crate::api::handlers::health_handlers;
use actix_web::web;

/// Configures all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1").service(
            web::scope("/health")
                .service(health_handlers::health_check)
                .service(health_handlers::database_health),
        ),
    );
}
