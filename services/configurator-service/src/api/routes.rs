use actix_web::web;
use crate::api::handlers::{network_handler, health_handler};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/networks").configure(network_handler::init_routes));
    cfg.route("/health", web::get().to(health_handler::health_check));
}
