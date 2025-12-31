use actix_web::{post, web, HttpResponse};

use crate::{
    application::dtos::registration::{RegisterRequest, ResendVerificationRequest, VerifyRequest},
    AppState,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(verify)
        .service(resend_verification);
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "Registration",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful, verification email sent"),
        (status = 400, description = "Invalid request data"),
        (status = 409, description = "User already exists")
    )
)]
#[post("/register")]
async fn register(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    match state.registration_service.register(req.into_inner()).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "message": "Registration successful. Please check your email for verification."
        })),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/verify",
    tag = "Registration",
    request_body = VerifyRequest,
    responses(
        (status = 200, description = "Email verified successfully"),
        (status = 400, description = "Invalid verification token"),
        (status = 410, description = "Verification link expired")
    )
)]
#[post("/verify")]
async fn verify(
    state: web::Data<AppState>,
    req: web::Json<VerifyRequest>,
) -> HttpResponse {
    match state.registration_service.verify(req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Email verified successfully. You can now log in."
        })),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/verify/resend",
    tag = "Registration",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email resent"),
        (status = 429, description = "Too many resend attempts"),
        (status = 404, description = "Registration not found")
    )
)]
#[post("/verify/resend")]
async fn resend_verification(
    state: web::Data<AppState>,
    req: web::Json<ResendVerificationRequest>,
) -> HttpResponse {
    match state.registration_service.resend_verification(req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Verification email resent successfully."
        })),
        Err(e) => e.error_response(),
    }
}