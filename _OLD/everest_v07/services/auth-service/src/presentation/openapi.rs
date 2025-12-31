use crate::application::dtos::{
    authentication::{AuthResponse, LoginRequest},
    health::HealthResponse,
    registration::{RegisterUserRequest, RegisterUserResponse},
};
use utoipa::OpenApi;

use crate::presentation::controllers::{
    admin_controller, authentication_controller, health_controller, invitation_controller,
    registration_controller,
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
        authentication_controller::refresh,
        authentication_controller::validate,
        admin_controller::list_users,
        admin_controller::get_user,
        admin_controller::create_user,
        admin_controller::update_user,
        admin_controller::delete_user,
        invitation_controller::create,
        invitation_controller::list,
        invitation_controller::get,
        invitation_controller::accept,
        invitation_controller::cancel
    ),
    components(
        schemas(
            HealthResponse,
            RegisterUserRequest,
            RegisterUserResponse,
            LoginRequest,
            AuthResponse
        )
    ),
    tags(
        (name = "Authentication", description = "Login and Token management"),
        (name = "Registration", description = "User onboarding"),
        (name = "Admin", description = "Administrative User Management"),
        (name = "Invitations", description = "User invitation management"),
        (name = "Health", description = "System status")
    )
)]
pub struct ApiDoc;
