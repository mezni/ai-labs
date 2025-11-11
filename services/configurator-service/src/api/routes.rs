use crate::api::handlers::{health_handlers, network_handlers};
use actix_web::web;

/// Configure all application routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/health")
                    .service(health_handlers::health_check)
                    .service(health_handlers::database_health),
            )
            .service(
                web::scope("/networks")
                    .service(network_handlers::get_network)
                    .service(network_handlers::create_network),
            ),
    );
}
