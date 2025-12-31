use crate::application::dtos::registration::*;
use crate::core::errors::AppError;
use crate::domain::services::RegistrationService;
use actix_web::{HttpRequest, HttpResponse, ResponseError, post, web};
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = RegisterResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Email already exists")
    ),
    tag = "Registration"
)]
#[post("/register")]
pub async fn register(
    service: web::Data<Arc<dyn RegistrationService>>,
    req: HttpRequest,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    if let Err(e) = body.0.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": e.to_string()
        }));
    }

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match service
        .register(
            body.email.clone(),
            body.username.clone(),
            body.password.clone(),
            body.first_name.clone(),
            body.last_name.clone(),
            body.phone.clone(),
            body.source.clone(),
            ip_address,
            user_agent,
        )
        .await
    {
        Ok(registration) => HttpResponse::Created().json(RegisterResponse {
            registration_id: registration.registration_id,
            email: registration.email,
            message: "Registration successful. Please check your email to verify your account."
                .to_string(),
        }),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/verify",
    request_body = VerifyRequest,
    responses(
        (status = 200, description = "Verification successful", body = VerifyResponse),
        (status = 400, description = "Invalid or expired token"),
        (status = 404, description = "Token not found")
    ),
    tag = "Registration"
)]
#[post("/verify")]
pub async fn verify(
    service: web::Data<Arc<dyn RegistrationService>>,
    body: web::Json<VerifyRequest>,
) -> HttpResponse {
    match service.verify(body.token.clone()).await {
        Ok(user) => HttpResponse::Ok().json(VerifyResponse {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            message: "Email verified successfully. You can now login.".to_string(),
        }),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/verify/resend",
    request_body = ResendRequest,
    responses(
        (status = 200, description = "Verification email resent", body = ResendResponse),
        (status = 400, description = "Cannot resend"),
        (status = 404, description = "Registration not found")
    ),
    tag = "Registration"
)]
#[post("/verify/resend")]
pub async fn resend_verification(
    service: web::Data<Arc<dyn RegistrationService>>,
    body: web::Json<ResendRequest>,
) -> HttpResponse {
    if let Err(e) = body.0.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": e.to_string()
        }));
    }

    match service.resend_verification(body.email.clone()).await {
        Ok(_) => HttpResponse::Ok().json(ResendResponse {
            message: "Verification email has been resent.".to_string(),
        }),
        Err(e) => e.error_response(),
    }
}
