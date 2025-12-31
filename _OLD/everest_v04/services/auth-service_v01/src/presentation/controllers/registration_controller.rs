use crate::application::register_dto::{RegisterRequest, RegisterResponse, ResendVerificationRequest, ResendVerificationResponse};
use crate::application::registration_service::RegistrationService;
use crate::core::state::AppState;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use utoipa;

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration created successfully", body = RegisterResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email or username already exists")
    ),
    tag = "Registration"
)]
pub async fn register(
    state: web::Data<AppState>,
    request: web::Json<RegisterRequest>,
) -> impl Responder {
    match RegistrationService::register(state, request.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/verify/resend",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email resent", body = ResendVerificationResponse),
        (status = 404, description = "Registration not found"),
        (status = 410, description = "Token expired"),
        (status = 429, description = "Too many requests")
    ),
    tag = "Registration"
)]
pub async fn resend_verification(
    state: web::Data<AppState>,
    request: web::Json<ResendVerificationRequest>,
) -> impl Responder {
    match RegistrationService::resend_verification(state, request.email.clone()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => err.error_response(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register")
            .route(web::post().to(register))
    )
    .service(
        web::resource("/verify/resend")
            .route(web::post().to(resend_verification))
    );
}