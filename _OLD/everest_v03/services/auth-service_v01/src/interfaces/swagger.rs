use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::application;
use crate::interfaces;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health & Status
        interfaces::http::health::health_check,
        interfaces::http::health::metrics,
        interfaces::http::health::version,
        // Registration
        interfaces::http::registration::create_registration,
        interfaces::http::registration::verify_registration,
        interfaces::http::registration::resend_verification,
        interfaces::http::registration::get_registration_status,
        interfaces::http::registration::cancel_registration,
    ),
    components(
        schemas(
            application::health::HealthResponse,
            application::health::VersionResponse,
            application::registration::RegistrationRequest,
            application::registration::RegistrationResponse,
            application::registration::VerifyRequest,
            application::registration::VerifyResponse,
            application::registration::ResendRequest,
            application::registration::RegistrationStatusResponse,
        )
    ),
    tags(
        (name = "Health & Status", description = "Service health and monitoring endpoints"),
        (name = "Registration", description = "User registration and verification endpoints"),
    ),
    info(
        title = "Auth API",
        version = "0.1.0",
        description = "Authentication and user management API",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT"
        )
    )
)]
pub struct ApiDoc;

pub fn configure_swagger(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
