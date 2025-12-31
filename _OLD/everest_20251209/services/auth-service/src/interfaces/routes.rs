use super::handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            // Health check
            .route("/health", web::get().to(handlers::health_check))
            // Public authentication endpoints
            .route("/register", web::post().to(handlers::register))
            .route("/login", web::post().to(handlers::login))
            .service(web::scope("/auth").route("/refresh", web::post().to(handlers::refresh_token)))
            // User management endpoints
            .service(
                web::scope("/users")
                    .route("", web::get().to(handlers::list_users))
                    .route("", web::post().to(handlers::create_user))
                    .route("/{user_id}", web::get().to(handlers::get_user))
                    .route("/{user_id}", web::put().to(handlers::update_user))
                    .route("/{user_id}", web::delete().to(handlers::deactivate_user))
                    .route(
                        "/{user_id}/change-password",
                        web::post().to(handlers::change_password),
                    ),
            ),
    );
}
