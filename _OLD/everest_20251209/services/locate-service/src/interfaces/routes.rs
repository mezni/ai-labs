use crate::interfaces::handlers;
use crate::middleware::{JwtAuth, RequireRole};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig, jwt_auth: JwtAuth) {
    cfg.service(
        web::scope("/api/v1")
            // Public endpoints (no authentication required)
            .service(
                web::scope("/stations")
                    .route("/nearby", web::get().to(handlers::get_nearby_stations)),
            )
            // Protected endpoints (authentication required)
            .service(
                web::scope("")
                    .wrap(jwt_auth.clone())
                    .service(
                        web::scope("/user").route("/info", web::get().to(handlers::get_user_info)),
                    )
                    // Review endpoints require "user" role
                    .service(
                        web::scope("/reviews")
                            .wrap(RequireRole::new("user"))
                            .route("", web::post().to(handlers::create_review))
                            .route(
                                "/station/{station_id}",
                                web::get().to(handlers::get_station_reviews),
                            )
                            .route("/{review_id}", web::put().to(handlers::update_review))
                            .route("/{review_id}", web::delete().to(handlers::delete_review)),
                    ),
            ),
    )
    .route("/health", web::get().to(handlers::health_check));
}
