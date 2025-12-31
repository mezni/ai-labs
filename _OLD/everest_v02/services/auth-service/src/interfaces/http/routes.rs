use crate::core::constants::API_PREFIX;
use crate::interfaces::http::{health_handler, user_registration_handler};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Everything inside this scope is prefixed with /api/v1 (from API_PREFIX)
    cfg.service(
        web::scope(API_PREFIX)
            // GET /api/v1/health
            .service(health_handler::get_health)
            // POST /api/v1/register
            .route(
                "/register",
                web::post().to(user_registration_handler::register_user),
            )
            // POST /api/v1/verify
            // This endpoint checks the token sent to the user's email
            .route(
                "/verify",
                web::post().to(user_registration_handler::verify_user),
            ),
    );
}
