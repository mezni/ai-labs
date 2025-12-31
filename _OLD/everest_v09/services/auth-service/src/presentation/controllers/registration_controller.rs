use crate::AppState;
use crate::application::dtos::registration::{
    RegisterRequest, RegisterResponse, ResendRequest, ResendResponse, VerifyRequest, VerifyResponse,
};
use crate::application::registration_service::RegistrationService;
use crate::core::errors::AppError;
use crate::domain::services::RegistrationService as RegistrationServiceTrait;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use validator::Validate;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user))
        .route("/verify", web::post().to(verify_registration))
        .route("/verify/resend", web::post().to(resend_verification));
}

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists")
    ),
    tag = "Registration"
)]
pub async fn register_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let svc = RegistrationService::new(state.into_inner());

    // Added "web" as default source since it's missing from your RegisterRequest DTO
    let registration = svc
        .register(
            body.email.clone(),
            body.username.clone(),
            body.password.clone(),
            body.first_name.clone(),
            body.last_name.clone(),
            body.phone.clone(),
            "web".to_string(), // Default source
            ip_address,
            user_agent,
        )
        .await?;

    Ok(HttpResponse::Created().json(RegisterResponse {
        registration_id: registration.registration_id,
        email: registration.email,
        message: "Registration successful. Please check your email.".into(),
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/verify",
    request_body = VerifyRequest,
    responses(
        (status = 200, description = "Account verified", body = VerifyResponse),
        (status = 400, description = "Invalid/Expired token")
    ),
    tag = "Registration"
)]
pub async fn verify_registration(
    state: web::Data<AppState>,
    body: web::Json<VerifyRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let svc = RegistrationService::new(state.into_inner());
    let user = svc.verify(body.token.clone()).await?;

    Ok(HttpResponse::Ok().json(VerifyResponse {
        user_id: user.user_id,
        email: user.email,
        username: user.username,
        message: "Email verified successfully.".into(),
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/verify/resend",
    request_body = ResendRequest,
    responses(
        (status = 200, description = "Verification email resent", body = ResendResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Registration"
)]
pub async fn resend_verification(
    state: web::Data<AppState>,
    body: web::Json<ResendRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let svc = RegistrationService::new(state.into_inner());
    svc.resend_verification(body.email.clone()).await?;

    Ok(HttpResponse::Ok().json(ResendResponse {
        message: "Verification email has been resent.".into(),
    }))
}
