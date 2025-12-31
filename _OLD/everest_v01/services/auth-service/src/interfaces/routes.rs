// src/interfaces/routes.rs
use crate::application::{AuditQueries, AuthService, UserService};
use crate::core::{JwtAuth, RequireRole};
use crate::interfaces::{
    admin_handlers, audit_handlers, auth_handlers, health_handlers, sync_handlers, user_handlers,
};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Health check (no auth)
    cfg.service(health_handlers::health_check);

    // Public auth endpoints (no auth)
    cfg.service(
        web::scope("/api/v1/auth")
            .service(auth_handlers::register)
            .service(auth_handlers::verify_email)
            .service(auth_handlers::login)
            .service(auth_handlers::refresh_token)
            .service(auth_handlers::request_password_reset),
    );

    // Authenticated auth endpoints
    cfg.service(
        web::scope("/api/v1/auth")
            .wrap(JwtAuth)
            .service(auth_handlers::change_password)
            .service(auth_handlers::logout),
    );

    // User endpoints (authenticated)
    cfg.service(
        web::scope("/api/v1/users")
            .wrap(JwtAuth)
            .service(user_handlers::get_my_profile)
            .service(user_handlers::update_my_profile),
    );

    // Admin user management endpoints
    cfg.service(
        web::scope("/api/v1/admin/users")
            .wrap(RequireRole::admin())
            .wrap(JwtAuth)
            .service(admin_handlers::create_internal_user)
            .service(user_handlers::list_users)
            .service(user_handlers::get_user)
            .service(admin_handlers::update_user)
            .service(admin_handlers::delete_user)
            .service(admin_handlers::reset_password),
    );

    // Admin audit endpoints
    cfg.service(
        web::scope("/api/v1/admin/login-audit")
            .wrap(RequireRole::admin())
            .wrap(JwtAuth)
            .service(audit_handlers::get_login_audit_logs),
    );

    // Admin Keycloak sync endpoints
    cfg.service(
        web::scope("/api/v1/admin/keycloak-sync")
            .wrap(RequireRole::admin())
            .wrap(JwtAuth)
            .service(sync_handlers::get_sync_stats)
            .service(sync_handlers::trigger_sync)
            .service(sync_handlers::sync_user),
    );
}

// Service factory for dependency injection
pub struct ServiceFactory;

impl ServiceFactory {
    pub fn create_auth_service(
        user_repo: std::sync::Arc<dyn crate::domain::repositories::UserRepository>,
        registration_repo: std::sync::Arc<dyn crate::domain::repositories::RegistrationRepository>,
        audit_repo: std::sync::Arc<dyn crate::domain::repositories::AuditLogRepository>,
        keycloak_client: std::sync::Arc<crate::infrastructure::KeycloakClient>,
        token_blacklist: std::sync::Arc<crate::infrastructure::TokenBlacklist>,
    ) -> AuthService {
        AuthService::new(
            user_repo,
            registration_repo,
            audit_repo,
            keycloak_client,
            token_blacklist,
        )
    }

    pub fn create_user_service(
        user_repo: std::sync::Arc<dyn crate::domain::repositories::UserRepository>,
        audit_repo: std::sync::Arc<dyn crate::domain::repositories::AuditLogRepository>,
        keycloak_client: std::sync::Arc<crate::infrastructure::KeycloakClient>,
    ) -> UserService {
        UserService::new(user_repo, audit_repo, keycloak_client)
    }

    pub fn create_audit_queries(
        audit_repo: std::sync::Arc<dyn crate::domain::repositories::AuditLogRepository>,
    ) -> AuditQueries {
        AuditQueries::new(audit_repo)
    }
}
