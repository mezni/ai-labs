use crate::application::dtos::authentication::{
    LoginRequest, LoginResponseDto, LogoutRequest, MessageResponse, RefreshTokenRequest,
    UserInfoDto,
};
use crate::application::dtos::health::HealthResponse;
use crate::application::dtos::registration::{
    RegisterRequest, RegisterResponse, ResendRequest, ResendResponse, VerifyRequest, VerifyResponse,
};
// Add Admin and Invitation DTOs
use crate::application::dtos::admin::{
    CreateUserRequest, UpdateUserRequest, UserListResponse, UserResponse,
};
use crate::presentation::controllers::{
    admin_controller, // Ensure this module is created and exported
    authentication_controller,
    health_controller,
    registration_controller,
};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_controller::health_check,
        registration_controller::register_user,
        registration_controller::verify_registration,
        registration_controller::resend_verification,
        authentication_controller::login,
        authentication_controller::logout,
        authentication_controller::refresh_token,
        // Add Admin paths
        admin_controller::list_users,
        admin_controller::get_user,
        admin_controller::create_user,
        admin_controller::update_user,
        admin_controller::delete_user,
    ),
    components(
        schemas(
            HealthResponse,
            RegisterRequest,
            RegisterResponse,
            VerifyRequest,
            VerifyResponse,
            ResendRequest,
            ResendResponse,
            LoginRequest,
            LoginResponseDto,
            UserInfoDto,
            RefreshTokenRequest,
            LogoutRequest,
            MessageResponse,
            // Add Admin schemas
            UserResponse,
            UserListResponse,
            CreateUserRequest,
            UpdateUserRequest,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Registration", description = "User registration and verification"),
        (name = "Authentication", description = "User authentication"),
        (name = "Admin", description = "Administrative user management"),
    ),
    info(
        title = "Authentication Service API",
        version = "1.0.0",
        description = "API for user registration, verification, authentication, and administration"
    )
)]
pub struct ApiDoc;

/// Helper struct to inject Bearer Token security into Swagger UI
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
