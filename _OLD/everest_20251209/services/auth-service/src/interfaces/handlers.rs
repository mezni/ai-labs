use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use validator::Validate;

use crate::application::{
    AuthService,
    dto::{
        ChangePasswordRequest, CreateUserRequest, HealthCheckResponse, LoginRequest,
        RefreshTokenRequest, RegisterRequest, RegisterResponse, UpdateUserRequest,
        UserListResponse,
    },
};
use crate::config::Config;
use crate::domain::{TokenResponse, User, UserRole};
use crate::infrastructure::{DomainError, KeycloakClient, PostgresUserRepository};

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthCheckResponse)
    ),
    tag = "Health"
)]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

/// Self-register a new user (role=USER, source=web)
#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Authentication"
)]
pub async fn register(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    request: web::Json<RegisterRequest>,
) -> Result<impl Responder, DomainError> {
    request
        .validate()
        .map_err(|e| DomainError::ValidationError(format!("Validation failed: {}", e)))?;

    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    let user = service.register(request.into_inner()).await?;

    Ok(HttpResponse::Created().json(RegisterResponse {
        user_id: user.user_id.to_string(),
        message: "User registered successfully. Please check your email for verification."
            .to_string(),
    }))
}

/// Admin creates a user (with role, network_id, station_id, source=internal)
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = ["ADMIN"])
    )
)]
pub async fn create_user(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    request: web::Json<CreateUserRequest>,
) -> Result<impl Responder, DomainError> {
    request
        .validate()
        .map_err(|e| DomainError::ValidationError(format!("Validation failed: {}", e)))?;

    // Verify role is not USER
    if request.role == UserRole::User {
        return Err(DomainError::ValidationError(
            "Cannot create USER role via admin endpoint. Users should self-register.".to_string(),
        ));
    }

    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    let created_by = "ADMIN001"; // TODO: Extract from JWT

    let user = service
        .create_user_by_admin(request.into_inner(), created_by)
        .await?;

    Ok(HttpResponse::Created().json(user))
}

/// Login user
#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Authentication"
)]
pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    request: web::Json<LoginRequest>,
) -> Result<impl Responder, DomainError> {
    request
        .validate()
        .map_err(|e| DomainError::ValidationError(format!("Validation failed: {}", e)))?;

    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    let token = service.login(request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(token))
}

/// Change password
#[utoipa::path(
    post,
    path = "/api/v1/users/{user_id}/change-password",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Invalid old password"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn change_password(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    request: web::Json<ChangePasswordRequest>,
) -> Result<impl Responder, DomainError> {
    let user_id = path.into_inner();

    request
        .validate()
        .map_err(|e| DomainError::ValidationError(format!("Validation failed: {}", e)))?;

    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    service
        .change_password(&user_id, request.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password changed successfully"
    })))
}

/// Refresh access token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = TokenResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Invalid refresh token"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    request: web::Json<RefreshTokenRequest>,
) -> Result<impl Responder, DomainError> {
    request
        .validate()
        .map_err(|e| DomainError::ValidationError(format!("Validation failed: {}", e)))?;

    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    let token = service.refresh_token(&request.refresh_token).await?;

    Ok(HttpResponse::Ok().json(token))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<impl Responder, DomainError> {
    let user_id = path.into_inner();

    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    let user = service.get_user_by_id(&user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}

/// List users
#[utoipa::path(
    get,
    path = "/api/v1/users",
    params(
        ("role" = Option<String>, Query, description = "Filter by role"),
        ("is_active" = Option<bool>, Query, description = "Filter by active status")
    ),
    responses(
        (status = 200, description = "List of users", body = UserListResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = ["ADMIN"])
    )
)]
pub async fn list_users(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<impl Responder, DomainError> {
    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    let role = query.get("role").and_then(|r| UserRole::from_str(r));
    let is_active = query.get("is_active").and_then(|a| a.parse::<bool>().ok());

    let users = service.list_users(role, is_active).await?;
    let total = users.len();

    Ok(HttpResponse::Ok().json(UserListResponse { users, total }))
}

/// Update user
#[utoipa::path(
    put,
    path = "/api/v1/users/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 400, description = "Validation error"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_user(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    request: web::Json<UpdateUserRequest>,
) -> Result<impl Responder, DomainError> {
    let user_id = path.into_inner();

    request
        .validate()
        .map_err(|e| DomainError::ValidationError(format!("Validation failed: {}", e)))?;

    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    let updated_by = &user_id; // TODO: Extract from JWT

    let user = service
        .update_user(&user_id, request.into_inner(), updated_by)
        .await?;

    Ok(HttpResponse::Ok().json(user))
}

/// Deactivate user
#[utoipa::path(
    delete,
    path = "/api/v1/users/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deactivated successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = ["ADMIN"])
    )
)]
pub async fn deactivate_user(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<impl Responder, DomainError> {
    let user_id = path.into_inner();

    let user_repo = Arc::new(PostgresUserRepository::new(pool.get_ref().clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.get_ref().clone()));
    let service = AuthService::new(user_repo, keycloak_client);

    let updated_by = "ADMIN001"; // TODO: Extract from JWT

    service.deactivate_user(&user_id, updated_by).await?;

    Ok(HttpResponse::NoContent().finish())
}
