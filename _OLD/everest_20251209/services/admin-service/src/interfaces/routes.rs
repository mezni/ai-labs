use actix_web::web;
use crate::interfaces::handlers;
use crate::middleware::{JwtAuth, RequireRole};

pub fn configure_routes(cfg: &mut web::ServiceConfig, jwt_auth: JwtAuth) {
    cfg.service(
        web::scope("/api/v1")
            // Middleware order: They are applied BOTTOM to TOP
            // So jwt_auth runs first, then RequireRole
            .wrap(RequireRole::new("admin"))  // Applied SECOND (checks role)
            .wrap(jwt_auth)                    // Applied FIRST (validates JWT and stores claims)
            // Network endpoints
            .service(
                web::scope("/networks")
                    .route("", web::post().to(handlers::create_network))
                    .route("", web::get().to(handlers::list_networks))
                    .route("/{network_id}", web::get().to(handlers::get_network))
                    .route("/{network_id}", web::put().to(handlers::update_network))
                    .route("/{network_id}", web::delete().to(handlers::delete_network))
                    .route("/{network_id}/stations", web::get().to(handlers::list_stations_by_network))
            )
            // Station endpoints
            .service(
                web::scope("/stations")
                    .route("", web::post().to(handlers::create_station))
                    .route("", web::get().to(handlers::list_stations))
                    .route("/{station_id}", web::get().to(handlers::get_station))
                    .route("/{station_id}", web::put().to(handlers::update_station))
                    .route("/{station_id}", web::delete().to(handlers::delete_station))
                    .route("/{station_id}/connectors", web::get().to(handlers::list_connectors_by_station))
            )
            // Connector endpoints
            .service(
                web::scope("/connectors")
                    .route("", web::post().to(handlers::create_connector))
                    .route("/{connector_id}", web::get().to(handlers::get_connector))
                    .route("/{connector_id}", web::put().to(handlers::update_connector))
                    .route("/{connector_id}", web::delete().to(handlers::delete_connector))
            )
    )
    .route("/health", web::get().to(handlers::health_check));
}