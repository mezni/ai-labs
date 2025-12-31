use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::handlers::health_check,
        crate::interfaces::handlers::register,
        crate::interfaces::handlers::create_user,
        crate::interfaces::handlers::login,
        crate::interfaces::handlers::change_password,
        crate::interfaces::handlers::refresh_token,
        crate::interfaces::handlers::get_user,
        crate::interfaces::handlers::list_users,
        crate::interfaces::handlers::update_user,
        crate::interfaces::handlers::deactivate_user,
    ),
    components(
        schemas(
            crate::domain::User,
            crate::domain::TokenResponse,
            crate::domain::UserRole,
            crate::domain::UserSource,
            crate::application::dto::RegisterRequest,
            crate::application::dto::CreateUserRequest,
            crate::application::dto::LoginRequest,
            crate::application::dto::ChangePasswordRequest,
            crate::application::dto::RefreshTokenRequest,
            crate::application::dto::UpdateUserRequest,
            crate::application::dto::HealthCheckResponse,
            crate::application::dto::RegisterResponse,
            crate::application::dto::UserListResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication and registration"),
        (name = "Users", description = "User management")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token from Keycloak authentication"))
                        .build(),
                ),
            );
        }

        openapi.info.title = "EV Charging Auth Service API".to_string();
        openapi.info.version = "1.0.0".to_string();
        openapi.info.description = Some(
            "REST API for user authentication and management using Keycloak\n\n\
            ## Keycloak Clients\n\n\
            ### auth-client (Public)\n\
            - Used for user authentication (login, refresh token)\n\
            - Public client, no secret required\n\
            - Direct access grants enabled\n\n\
            ### backend-admin (Service Account)\n\
            - Used for admin operations (create users, assign roles)\n\
            - Service account with client credentials grant\n\
            - Has admin permissions in Keycloak\n\n\
            ## User Types\n\n\
            ### Self-Registered Users (role=USER, source=web)\n\
            - Register via `/api/v1/register`\n\
            - Uses auth-client for authentication\n\
            - No network_id or station_id\n\n\
            ### Admin-Created Users (role=ADMIN/PARTNER/OPERATOR, source=internal)\n\
            - Created via `/api/v1/users` by admins\n\
            - Uses backend-admin for creation\n\
            - Authenticates via auth-client\n\
            - Have network_id and/or station_id\n\
            - Roles assigned in Keycloak"
                .to_string(),
        );
    }
}
