// src/interfaces/openapi.rs
use crate::application::*;
use crate::interfaces::{
    admin_handlers, audit_handlers, auth_handlers, health_handlers, sync_handlers, user_handlers,
};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health
        health_handlers::health_check,
        
        // Auth
        auth_handlers::register,
        auth_handlers::verify_email,
        auth_handlers::login,
        auth_handlers::refresh_token,
        auth_handlers::change_password,
        auth_handlers::request_password_reset,
        auth_handlers::logout,
        
        // Users
        user_handlers::get_my_profile,
        user_handlers::update_my_profile,
        user_handlers::list_users,
        user_handlers::get_user,
        
        // Admin
        admin_handlers::create_internal_user,
        admin_handlers::update_user,
        admin_handlers::delete_user,
        admin_handlers::reset_password,
        
        // Audit
        audit_handlers::get_login_audit_logs,
        
        // Sync
        sync_handlers::get_sync_stats,
        sync_handlers::trigger_sync,
        sync_handlers::sync_user,
    ),
    components(
        schemas(
            // Health
            health_handlers::HealthResponse,
            
            // Auth DTOs
            RegisterRequest,
            RegisterResponse,
            VerifyEmailRequest,
            VerifyEmailResponse,
            LoginRequest,
            LoginResponse,
            UserInfo,
            RefreshTokenRequest,
            TokenResponse,
            ChangePasswordRequest,
            RequestPasswordResetRequest,
            AuthSuccessResponse,
            
            // User DTOs
            UserResponse,
            UserDetailResponse,
            CreateInternalUserRequest,
            UpdateProfileRequest,
            AdminUpdateUserRequest,
            ListUsersRequest,
            PaginatedUsersResponse,
            DeleteUserResponse,
            AdminResetPasswordRequest,
            SuccessResponse,
            ErrorResponse,
            
            // Audit DTOs
            LoginAuditLogResponse,
            GetAuditLogsRequest,
            PaginatedAuditLogsResponse,
            
            // Sync DTOs
            sync_handlers::SyncStatsResponse,
            sync_handlers::TriggerSyncResponse,
            sync_handlers::SyncUserResponse,
        )
    ),
    tags(
        (name = "Health", description = "Service health check"),
        (name = "Authentication", description = "User authentication and registration"),
        (name = "Users", description = "User profile management"),
        (name = "Admin - Users", description = "Administrative user management"),
        (name = "Admin - Audit", description = "Audit log management"),
        (name = "Admin - Sync", description = "Keycloak synchronization")
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Auth Service API",
        version = env!("CARGO_PKG_VERSION"),
        description = "Authentication and Authorization Service - Single security backbone of the platform",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token obtained from /api/v1/auth/login endpoint"))
                        .build(),
                ),
            )
        }
    }
}
