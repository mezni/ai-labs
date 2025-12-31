use crate::application::dtos::*;
use crate::domain::entities::{Role, UserSource};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::handlers::register,
        crate::interfaces::handlers::health,
        crate::interfaces::handlers::get_me,
        crate::interfaces::handlers::update_me,
        crate::interfaces::handlers::create_internal_user,
        crate::interfaces::handlers::list_users,
        crate::interfaces::handlers::update_user,
        crate::interfaces::handlers::delete_user,
    ),
    components(
        schemas(
            RegisterRequest,
            RegisterResponse,
            CreateInternalUserRequest,
            UpdateUserRequest,
            UpdateMeRequest,
            UserResponse,
            ListUsersResponse,
            HealthResponse,
            Role,
            UserSource,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "admin", description = "Admin endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Auth Service API",
        version = "1.0.0",
        description = "Authentication and Authorization Service"
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}