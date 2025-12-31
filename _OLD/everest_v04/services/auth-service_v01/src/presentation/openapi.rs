use crate::application::{
    login_dto::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse},
    register_dto::{RegisterRequest, RegisterResponse, ResendVerificationRequest, ResendVerificationResponse},
    verify_dto::{VerifyRequest, VerifyResponse},
};
use crate::core::errors::ErrorResponse;
use crate::presentation::controllers::{
    authentication_controller, registration_controller,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        registration_controller::register,
        registration_controller::resend_verification,
        authentication_controller::verify,
        authentication_controller::login,
        authentication_controller::refresh_token,
    ),
    components(
        schemas(
            RegisterRequest,
            RegisterResponse,
            ResendVerificationRequest,
            ResendVerificationResponse,
            VerifyRequest,
            VerifyResponse,
            LoginRequest,
            LoginResponse,
            RefreshTokenRequest,
            RefreshTokenResponse,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Registration", description = "User registration endpoints"),
        (name = "Authentication", description = "Authentication endpoints")
    ),
    info(
        title = "Authentication Service API",
        version = "1.0.0",
        description = "RESTful API for user registration and authentication with Keycloak integration"
    )
)]
pub struct ApiDoc;

pub fn create_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}