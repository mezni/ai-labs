use utoipa::OpenApi;

use crate::application::dtos::authentication::{
    LoginRequest, LoginResponseDto, LogoutRequest, MessageResponse, RefreshTokenRequest,
};
use crate::application::dtos::health::HealthResponse;
use crate::application::dtos::registration::{
    RegisterRequest, RegisterResponse, ResendRequest, ResendResponse, VerifyRequest, VerifyResponse,
};

use crate::presentation::controllers::{
    authentication_controller, health_controller, registration_controller,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health
        health_controller::health_check,
        // Registration
        registration_controller::register_user,
        registration_controller::verify_registration,
        registration_controller::resend_verification,
        // Authentication
        authentication_controller::login,
        authentication_controller::logout,
        authentication_controller::refresh_token,
    ),
    components(
        schemas(
            // Health
            HealthResponse,
            // Registration
            RegisterRequest,
            RegisterResponse,
            VerifyRequest,
            VerifyResponse,
            ResendRequest,
            ResendResponse,
            // Authentication
            LoginRequest,
            LoginResponseDto,
            LogoutRequest,
            RefreshTokenRequest,
            MessageResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Registration", description = "User registration and verification"),
        (name = "Authentication", description = "Login, logout, refresh token endpoints"),
    ),
    info(
        title = "Authentication Service API",
        version = "1.0.0",
        description = "API for user registration, authentication, and health monitoring"
    )
)]
pub struct ApiDoc;
