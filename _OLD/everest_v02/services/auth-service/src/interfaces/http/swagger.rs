use crate::core::constants::APP_VERSION;
use utoipa::OpenApi;

// Import the DTOs from the application layer
use crate::application::health_service::HealthReport;
use crate::application::user_registration_dto::{
    RegisterUserRequest,
    RegisterUserResponse,
    VerifyRegistrationRequest,
    VerifyRegistrationResponse, // Added these
};

// Import the Handlers from the interface layer
use crate::interfaces::http::{health_handler, user_registration_handler};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_handler::get_health,
        user_registration_handler::register_user,
        user_registration_handler::verify_user // Added path handler
    ),
    components(
        schemas(
            HealthReport,
            RegisterUserRequest,
            RegisterUserResponse,
            VerifyRegistrationRequest,  // Added schema
            VerifyRegistrationResponse   // Added schema
        )
    ),
    info(
        title = "Auth-Service API",
        version = APP_VERSION,
        description = "Modern Rust API with Clean Architecture"
    ),
    tags(
        (name = "System", description = "Diagnostic endpoints"),
        (name = "Registration", description = "User self-registration workflow")
    )
)]
pub struct ApiDoc;
