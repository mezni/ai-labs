use crate::application::dtos::{
    //    authentication::{AuthResponse, LoginRequest},
    health::HealthResponse,
    registration::{RegisterRequest, RegisterResponse},
};
use utoipa::OpenApi;

use crate::presentation::controllers::{
    health_controller,
    registration_controller, // admin_controller, authentication_controller, invitation_controller, registration_controller,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_controller::health_check,
        registration_controller::register_user,
        registration_controller::verify_registration,
        registration_controller::resend_verification,
//        authentication_controller::login,
//        authentication_controller::logout,
//        authentication_controller::refresh,
//        authentication_controller::validate,
//        admin_controller::list_users,
//        admin_controller::get_user,
//        admin_controller::create_user,
//        admin_controller::update_user,
//        admin_controller::delete_user,
//        invitation_controller::create,
//        invitation_controller::list,
//        invitation_controller::get,
//        invitation_controller::accept,
//        invitation_controller::cancel
    ),
    components(
        schemas(
            HealthResponse,
            RegisterRequest,
            RegisterResponse,
//            LoginRequest,
//            AuthResponse
        )
    ),
    tags(
//        (name = "Authentication", description = "Login and Token management"),
//        (name = "Admin", description = "Administrative User Management"),
//        (name = "Invitations", description = "User invitation management"),
        (name = "Health", description = "System status"),
                (name = "Registration", description = "User onboarding")
    )
)]
pub struct ApiDoc;
