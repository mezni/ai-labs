use crate::AppState;
use crate::application::authentication_dto::*;
use crate::application::authentication_service::AuthenticationService;
// Add ResponseError here
use actix_web::{HttpResponse, Responder, ResponseError, web};

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses((status = 200, body = LoginResponse)),
    tag = "Authentication"
)]
pub async fn login(state: web::Data<AppState>, req: web::Json<LoginRequest>) -> impl Responder {
    match AuthenticationService::login(state, req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => err.error_response(), // This will now compile
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/verify",
    request_body = VerifyRequest,
    responses((status = 200, body = VerifyResponse)),
    tag = "Authentication"
)]
pub async fn verify(state: web::Data<AppState>, req: web::Json<VerifyRequest>) -> impl Responder {
    match AuthenticationService::verify(state, req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses((status = 200, body = RefreshTokenResponse)),
    tag = "Authentication"
)]
pub async fn refresh_token(
    state: web::Data<AppState>,
    req: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    match AuthenticationService::refresh_token(state, req.refresh_token.clone()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => err.error_response(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/verify", web::post().to(verify))
            .route("/refresh", web::post().to(refresh_token)),
    );
}
