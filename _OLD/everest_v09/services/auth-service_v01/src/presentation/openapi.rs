//use crate::application::dtos::authentication::*;
use crate::application::dtos::health::*;
//use crate::application::dtos::registration::*;
use crate::presentation::controllers::{
    health_controller::*,
    // authentication_controller::*, health_controller::*, registration_controller::*,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
//        register,
//        verify,
//        resend_verification,
//        login,
//        logout,
//        refresh_token,
    ),
    components(
        schemas(
            HealthResponse,
//            RegisterRequest,
//            RegisterResponse,
//            VerifyRequest,
//            VerifyResponse,
//            ResendRequest,
//            ResendResponse,
//            LoginRequest,
//            LoginResponseDto,
//            UserInfoDto,
//            RefreshTokenRequest,
//            LogoutRequest,
//            MessageResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
//        (name = "Registration", description = "User registration and verification"),
//        (name = "Authentication", description = "User authentication"),
    ),
    info(
        title = "Authentication Service API",
        version = "1.0.0",
        description = "API for user registration, verification, and authentication"
    )
)]
pub struct ApiDoc;
