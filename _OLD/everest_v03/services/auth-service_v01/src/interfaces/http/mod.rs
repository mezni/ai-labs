//pub mod auth;
pub mod health;
//pub mod password;
pub mod registration;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health & Status
            .service(health::health_check)
            .service(health::metrics)
            .service(health::version)
            // Registration
            .service(registration::create_registration)
            .service(registration::verify_registration)
            .service(registration::resend_verification)
            .service(registration::get_registration_status)
            .service(registration::cancel_registration),
    );
}
