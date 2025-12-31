use crate::application::authentication_service::AuthenticationService;
use crate::application::login_dto::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse};
use crate::application::verify_dto::{VerifyRequest, VerifyResponse};
use crate::core::state::AppState;
use actix_web::{web, HttpResponse, Responder, ResponseError};

#[utoipa::path(
    post,
    path = "/api/v1/verify",
    request_body = VerifyRequest,
    responses(
        (status = 200, description = "Account verified successfully", body = VerifyResponse),
        (status = 400, description = "Invalid or expired token"),
        (status = 410, description = "Token expired")
    ),
    tag = "Authentication"
)]
pub async fn verify(
    state: web::Data<AppState>,
    request: web::Json<VerifyRequest>,
) -> impl Responder {
    match AuthenticationService::verify(state, request.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Authentication"
)]
pub async fn login(
    state: web::Data<AppState>,
    request: web::Json<LoginRequest>,
) -> impl Responder {
    match AuthenticationService::login(state, request.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshTokenResponse),
        (status = 401, description = "Invalid refresh token"),
        (status = 410, description = "Token expired")
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    state: web::Data<AppState>,
    request: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    match AuthenticationService::refresh_token(state, request.refresh_token.clone()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => err.error_response(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/verify")
            .route(web::post().to(verify))
    )
    .service(
        web::scope("/auth")
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/refresh").route(web::post().to(refresh_token)))
    );
}