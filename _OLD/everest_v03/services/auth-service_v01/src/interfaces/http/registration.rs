use crate::application::common::MessageResponse;
use crate::application::registration::{
    RegistrationRequest, RegistrationResponse, RegistrationStatusResponse, ResendRequest,
    VerifyRequest, VerifyResponse,
};
use actix_web::{HttpResponse, Responder, delete, get, post, web};

/// Create a new user registration
#[utoipa::path(
    post,
    path = "/api/v1/register",
    tag = "Registration",
    request_body = RegistrationRequest,
    responses(
        (status = 201, description = "Registration created successfully", body = RegistrationResponse),
        (status = 400, description = "Invalid request")
    )
)]
#[post("/register")]
pub async fn create_registration(req: web::Json<RegistrationRequest>) -> impl Responder {
    // TODO: Implement registration logic
    let response = RegistrationResponse {
        id: "XXX".to_string(),
        email: req.email.clone(),
        username: req.username.clone(),
        status: "pending_verification".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    HttpResponse::Created().json(response)
}

/// Verify registration token
#[utoipa::path(
    post,
    path = "/api/v1/verify",
    tag = "Registration",
    request_body = VerifyRequest,
    responses(
        (status = 200, description = "Token verified successfully", body = VerifyResponse),
        (status = 400, description = "Invalid or expired token")
    )
)]
#[post("/verify")]
pub async fn verify_registration(req: web::Json<VerifyRequest>) -> impl Responder {
    // TODO: Implement verification logic
    let response = VerifyResponse {
        success: true,
        message: "Registration verified successfully".to_string(),
    };
    HttpResponse::Ok().json(response)
}

/// Resend verification email
#[utoipa::path(
    post,
    path = "/api/v1/verify/resend",
    tag = "Registration",
    request_body = ResendRequest,
    responses(
        (status = 200, description = "Verification email sent", body = MessageResponse),
        (status = 404, description = "Registration not found")
    )
)]
#[post("/verify/resend")]
pub async fn resend_verification(req: web::Json<ResendRequest>) -> impl Responder {
    // TODO: Implement resend logic
    let response = MessageResponse {
        message: "Verification email sent".to_string(),
    };
    HttpResponse::Ok().json(response)
}

/// Get registration status
#[utoipa::path(
    get,
    path = "/api/v1/register/{id}",
    tag = "Registration",
    params(
        ("id" = String, Path, description = "Registration ID")
    ),
    responses(
        (status = 200, description = "Registration status retrieved", body = RegistrationStatusResponse),
        (status = 404, description = "Registration not found")
    )
)]
#[get("/register/{id}")]
pub async fn get_registration_status(path: web::Path<String>) -> impl Responder {
    // TODO: Implement status retrieval logic
    let response = RegistrationStatusResponse {
        id: path.into_inner(),
        status: "pending_verification".to_string(),
        verified: false,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    HttpResponse::Ok().json(response)
}

/// Cancel registration
#[utoipa::path(
    delete,
    path = "/api/v1/register/{id}",
    tag = "Registration",
    params(
        ("id" = String, Path, description = "Registration ID")
    ),
    responses(
        (status = 200, description = "Registration cancelled", body = MessageResponse),
        (status = 404, description = "Registration not found")
    )
)]
#[delete("/register/{id}")]
pub async fn cancel_registration(path: web::Path<String>) -> impl Responder {
    // TODO: Implement cancellation logic
    let response = MessageResponse {
        message: format!("Registration {} cancelled", path.into_inner()),
    };
    HttpResponse::Ok().json(response)
}
