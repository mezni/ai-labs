pub mod authentication_controller;
pub mod registration_controller;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(registration_controller::configure)
            .configure(authentication_controller::configure),
    );
}