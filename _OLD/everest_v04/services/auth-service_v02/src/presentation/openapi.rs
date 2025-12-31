use crate::application::authentication_dto::*;
use crate::application::registration_dto::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::controllers::authentication_controller::login,
        crate::presentation::controllers::authentication_controller::verify,
        crate::presentation::controllers::authentication_controller::refresh_token,
        crate::presentation::controllers::health_controller::get_health,
        crate::presentation::controllers::registration_controller::register,
        crate::presentation::controllers::registration_controller::resend_verification,
    ),
    components(schemas(
        Metadata,
        RegisterRequest,
        RegisterResponse,
        ResendVerificationRequest,
        ResendVerificationResponse,
        LoginRequest,
        LoginResponse,
        AuthMetadata,
        RefreshTokenRequest,
        RefreshTokenResponse,
        VerifyRequest,
        VerifyResponse,
        VerifyMetadata
    ))
)]
pub struct ApiDoc;

pub fn configure_openapi(cfg: &mut actix_web::web::ServiceConfig) {
    use utoipa_swagger_ui::SwaggerUi;
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
