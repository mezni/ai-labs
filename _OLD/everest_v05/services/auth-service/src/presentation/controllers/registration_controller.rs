use crate::AppState;
use crate::application::dtos::registration::{
    RegisterUserRequest, RegisterUserResponse, ResendVerificationRequest, VerifyUserRequest,
    VerifyUserResponse,
};
use crate::application::registration_service::RegistrationService;
use crate::core::errors::AppError;
use actix_web::{HttpResponse, Responder, web};
use validator::Validate;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/registration")
            .route("/register", web::post().to(register_user))
            .route("/verify", web::post().to(verify_registration))
            .route("/verify/resend", web::post().to(resend_verification)),
    );
}

#[utoipa::path(
    post,
    path = "/api/v1/registration/register",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterUserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Registration"
)]
pub async fn register_user(
    state: web::Data<AppState>,
    body: web::Json<RegisterUserRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let svc = RegistrationService::new(state.into_inner());
    let message = svc.register_user(body.into_inner()).await;

    Ok(HttpResponse::Created().json(RegisterUserResponse { message }))
}

#[utoipa::path(
    post,
    path = "/api/v1/registration/verify",
    request_body = VerifyUserRequest,
    responses(
        (status = 200, description = "Account verified", body = VerifyUserResponse),
        (status = 400, description = "Invalid token"),
        (status = 401, description = "Token expired"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Registration"
)]
pub async fn verify_registration(
    state: web::Data<AppState>,
    body: web::Json<VerifyUserRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let svc = RegistrationService::new(state.into_inner());
    let message = svc.verify_registration(body.into_inner()).await;

    Ok(HttpResponse::Ok().json(VerifyUserResponse {
        status: "success".into(),
        message,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/registration/verify/resend",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email resent", body = RegisterUserResponse),
        (status = 404, description = "User not found"),
        (status = 429, description = "Too many requests"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Registration"
)]
pub async fn resend_verification(
    state: web::Data<AppState>,
    body: web::Json<ResendVerificationRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let svc = RegistrationService::new(state.into_inner());
    let message = svc.resend_verification(body.into_inner()).await;

    Ok(HttpResponse::Ok().json(RegisterUserResponse { message }))
}
