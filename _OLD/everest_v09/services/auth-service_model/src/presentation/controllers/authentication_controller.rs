use crate::application::dtos::authentication::*;
use crate::domain::services::AuthenticationService;
use actix_web::{HttpResponse, ResponseError, post, web};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponseDto),
        (status = 401, description = "Invalid credentials"),
        (status = 403, description = "Account not verified or inactive")
    ),
    tag = "Authentication"
)]
#[post("/auth/login")]
pub async fn login(
    service: web::Data<Arc<dyn AuthenticationService>>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    match service
        .login(body.username.clone(), body.password.clone())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(LoginResponseDto {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            token_type: response.token_type,
            expires_in: response.expires_in,
            user: UserInfoDto {
                user_id: response.user.user_id,
                email: response.user.email,
                username: response.user.username,
                role: response.user.role,
            },
        }),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logout successful", body = MessageResponse),
        (status = 401, description = "Invalid token")
    ),
    tag = "Authentication"
)]
#[post("/auth/logout")]
pub async fn logout(
    service: web::Data<Arc<dyn AuthenticationService>>,
    body: web::Json<LogoutRequest>,
) -> HttpResponse {
    match service.logout(body.refresh_token.clone()).await {
        Ok(_) => HttpResponse::Ok().json(MessageResponse {
            message: "Logged out successfully".to_string(),
        }),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = LoginResponseDto),
        (status = 401, description = "Invalid refresh token")
    ),
    tag = "Authentication"
)]
#[post("/auth/refresh")]
pub async fn refresh_token(
    service: web::Data<Arc<dyn AuthenticationService>>,
    body: web::Json<RefreshTokenRequest>,
) -> HttpResponse {
    match service.refresh_token(body.refresh_token.clone()).await {
        Ok(response) => HttpResponse::Ok().json(LoginResponseDto {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            token_type: response.token_type,
            expires_in: response.expires_in,
            user: UserInfoDto {
                user_id: response.user.user_id,
                email: response.user.email,
                username: response.user.username,
                role: response.user.role,
            },
        }),
        Err(e) => e.error_response(),
    }
}
