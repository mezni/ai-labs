use crate::api::handlers::{health, networks};
use crate::core::constants::API_PREFIX;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(API_PREFIX)
            // Public routes
            .route("/health", web::get().to(health::health_check))
            .route("/health/pool", web::get().to(health::pool_stats))
            .route("/networks", web::get().to(networks::get_all_networks))
            .route(
                "/networks/{network_id}",
                web::get().to(networks::get_network_by_id),
            )
            // Protected routes (require authentication)
            .route("/networks", web::post().to(networks::create_network))
            .route(
                "/networks/{network_id}",
                web::put().to(networks::update_network),
            )
            .route(
                "/networks/{network_id}",
                web::delete().to(networks::delete_network),
            ),
    );
}
