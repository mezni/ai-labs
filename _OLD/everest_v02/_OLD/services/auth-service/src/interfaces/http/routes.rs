use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::{register_user, verification_callback};
use crate::core::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        register_user::register_user,
        verification_callback::verify_callback,
    ),
    components(schemas(
        register_user::RegisterUserRequest,
        register_user::RegisterUserResponse,
        verification_callback::KeycloakVerificationCallback,
    )),
    tags(
        (name = "Registration", description = "User self-registration endpoints"),
        (name = "Verification", description = "Email verification callbacks")
    ),
    info(
        title = "Auth Service API",
        version = "1.0.0",
        description = "Authentication service supporting user self-registration with Keycloak"
    )
)]
struct ApiDoc;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/register", web::post().to(register_user::register_user))
            .route(
                "/verify/callback",
                web::post().to(verification_callback::verify_callback),
            )
            .route("/health", web::get().to(health_check))
            .route("/ready", web::get().to(readiness_check)),
    )
    .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()));
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "auth-service",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn readiness_check(state: web::Data<AppState>) -> HttpResponse {
    // Check database connection
    let db_ready = sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .is_ok();

    if !db_ready {
        return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "reason": "database_unavailable"
        }));
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "checks": {
            "database": "ok"
        }
    }))
}
