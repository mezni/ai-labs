use utoipa::OpenApi;

use crate::application::dtos::{
    admin::{CreateUserRequest, UpdateUserRequest, UserResponse},
    authentication::{LoginRequest, LoginResponse, RefreshRequest, ValidateResponse},
    health::HealthResponse,
    invitation::{
        AcceptInvitationRequest, CreateInvitationRequest, InvitationDetailResponse,
        InvitationResponse,
    },
    registration::{RegisterRequest, ResendVerificationRequest, VerifyRequest},
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Authentication Service API",
        version = "1.0.0",
        description = "RESTful API for user authentication and management with Keycloak integration"
    ),
    paths(
        crate::presentation::controllers::health_controller::health_check,
        crate::presentation::controllers::registration_controller::register,
        crate::presentation::controllers::registration_controller::verify,
        crate::presentation::controllers::registration_controller::resend_verification,
        crate::presentation::controllers::authentication_controller::login,
        crate::presentation::controllers::authentication_controller::logout,
        crate::presentation::controllers::authentication_controller::refresh,
        crate::presentation::controllers::authentication_controller::validate,
        crate::presentation::controllers::admin_controller::list_users,
        crate::presentation::controllers::admin_controller::get_user,
        crate::presentation::controllers::admin_controller::create_user,
        crate::presentation::controllers::admin_controller::update_user,
        crate::presentation::controllers::admin_controller::delete_user,
        crate::presentation::controllers::invitation_controller::create_invitation,
        crate::presentation::controllers::invitation_controller::list_invitations,
        crate::presentation::controllers::invitation_controller::get_invitation,
        crate::presentation::controllers::invitation_controller::accept_invitation,
        crate::presentation::controllers::invitation_controller::cancel_invitation,
    ),
    components(
        schemas(
            HealthResponse,
            RegisterRequest,
            VerifyRequest,
            ResendVerificationRequest,
            LoginRequest,
            LoginResponse,
            RefreshRequest,
            ValidateResponse,
            CreateUserRequest,
            UpdateUserRequest,
            UserResponse,
            CreateInvitationRequest,
            InvitationResponse,
            InvitationDetailResponse,
            AcceptInvitationRequest,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Registration", description = "User registration and verification"),
        (name = "Authentication", description = "User authentication and token management"),
        (name = "Admin", description = "Administrative user management"),
        (name = "Invitations", description = "Invitation management"),
    )
)]
pub struct ApiDoc;

pub fn create_openapi_docs() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}