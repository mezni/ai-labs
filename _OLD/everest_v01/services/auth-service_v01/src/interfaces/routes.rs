use actix_web::web;
use utoipa_swagger_ui::SwaggerUi;
use crate::interfaces::handlers;
use crate::interfaces::openapi::ApiDoc;
use utoipa::OpenApi;
use actix_web::HttpResponse;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // your endpoints
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(handlers::register)),
            )
            .service(
                web::scope("/users")
                    .route("/me", web::get().to(handlers::get_me))
                    .route("/me", web::put().to(handlers::update_me)),
            )
            .service(
                web::scope("/admin/users")
                    .route("", web::post().to(handlers::create_internal_user))
                    .route("", web::get().to(handlers::list_users))
                    .route("/{id}", web::put().to(handlers::update_user))
                    .route("/{id}", web::delete().to(handlers::delete_user)),
            )
            .route("/health", web::get().to(handlers::health))
            // Serve OpenAPI JSON
            .route("/openapi.json", web::get().to(|| async {
                HttpResponse::Ok().json(ApiDoc::openapi())
            }))
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api/v1/openapi.json", ApiDoc::openapi()),
            )
    );
}
