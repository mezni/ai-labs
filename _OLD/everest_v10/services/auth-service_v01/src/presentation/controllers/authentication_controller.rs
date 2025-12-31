use crate::AppState;
use crate::application::authentication_service::AuthenticationService;
use crate::application::dtos::authentication::*;
use crate::domain::services::AuthenticationService as AuthServiceTrait;
use actix_web::{HttpResponse, Responder, ResponseError, web};
use validator::Validate;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // We wrap routes in a scope here so lib.rs stays clean
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh_token)),
    );
}

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
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginRequest>) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    let service = AuthenticationService::new(state.into_inner());

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
                role: format!("{:?}", response.user.role),
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
pub async fn logout(state: web::Data<AppState>, body: web::Json<LogoutRequest>) -> impl Responder {
    let service = AuthenticationService::new(state.into_inner());

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
pub async fn refresh_token(
    state: web::Data<AppState>,
    body: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    let service = AuthenticationService::new(state.into_inner());

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
                role: format!("{:?}", response.user.role),
            },
        }),
        Err(e) => e.error_response(),
    }
}
