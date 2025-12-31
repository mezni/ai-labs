// src/interfaces/auth_handlers.rs
use crate::application::{
    AuthService, AuthSuccessResponse, ChangePasswordRequest, LoginRequest, LoginResponse,
    RefreshTokenRequest, RegisterRequest, RegisterResponse, RequestPasswordResetRequest,
    TokenResponse, VerifyEmailRequest, VerifyEmailResponse,
};
use crate::core::{AppError, extract_claims};
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use validator::Validate;

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = RegisterResponse),
        (status = 400, description = "Invalid request"),
        (status = 409, description = "Email or username already exists")
    )
)]
#[post("/register")]
pub async fn register(
    auth_service: web::Data<AuthService>,
    request: web::Json<RegisterRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // Validate request
    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    // Extract IP and user agent
    let ip_address = http_req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let user_agent = http_req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let response = auth_service
        .commands
        .register(request.into_inner(), ip_address, user_agent)
        .await?;

    Ok(HttpResponse::Created().json(response))
}

/// Verify email with token
#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-email",
    tag = "Authentication",
    request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified successfully", body = VerifyEmailResponse),
        (status = 400, description = "Invalid or expired token"),
        (status = 404, description = "Token not found")
    )
)]
#[post("/verify-email")]
pub async fn verify_email(
    auth_service: web::Data<AuthService>,
    request: web::Json<VerifyEmailRequest>,
) -> Result<impl Responder, AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let response = auth_service
        .commands
        .verify_email(request.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Login
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 403, description = "Account disabled")
    )
)]
#[post("/login")]
pub async fn login(
    auth_service: web::Data<AuthService>,
    request: web::Json<LoginRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let ip_address = http_req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let user_agent = http_req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let response = auth_service
        .commands
        .login(request.into_inner(), ip_address, user_agent)
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Refresh token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = TokenResponse),
        (status = 401, description = "Invalid refresh token")
    )
)]
#[post("/refresh")]
pub async fn refresh_token(
    auth_service: web::Data<AuthService>,
    request: web::Json<RefreshTokenRequest>,
) -> Result<impl Responder, AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let response = auth_service
        .commands
        .refresh_token(request.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Change password (authenticated)
#[utoipa::path(
    post,
    path = "/api/v1/auth/change-password",
    tag = "Authentication",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully", body = AuthSuccessResponse),
        (status = 401, description = "Invalid old password or not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/change-password")]
pub async fn change_password(
    auth_service: web::Data<AuthService>,
    request: web::Json<ChangePasswordRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    auth_service
        .commands
        .change_password(claims.user_id, request.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(AuthSuccessResponse {
        message: "Password changed successfully".to_string(),
    }))
}

/// Request password reset
#[utoipa::path(
    post,
    path = "/api/v1/auth/request-password-reset",
    tag = "Authentication",
    request_body = RequestPasswordResetRequest,
    responses(
        (status = 200, description = "Password reset email sent", body = AuthSuccessResponse),
        (status = 404, description = "User not found")
    )
)]
#[post("/request-password-reset")]
pub async fn request_password_reset(
    auth_service: web::Data<AuthService>,
    request: web::Json<RequestPasswordResetRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let ip_address = http_req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    auth_service
        .commands
        .request_password_reset(request.into_inner(), ip_address)
        .await?;

    Ok(HttpResponse::Ok().json(AuthSuccessResponse {
        message: "If an account exists with this email, a password reset link has been sent."
            .to_string(),
    }))
}

/// Logout (authenticated)
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Logged out successfully", body = AuthSuccessResponse),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/logout")]
pub async fn logout(
    auth_service: web::Data<AuthService>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    // Extract token from Authorization header
    let token = http_req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing authorization token".to_string()))?
        .to_string();

    auth_service
        .commands
        .logout(claims.user_id, token, None)
        .await?;

    Ok(HttpResponse::Ok().json(AuthSuccessResponse {
        message: "Logged out successfully".to_string(),
    }))
}
