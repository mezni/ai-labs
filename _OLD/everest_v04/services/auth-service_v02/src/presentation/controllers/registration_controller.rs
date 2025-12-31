use crate::AppState;
use crate::application::registration_dto::RegisterRequest;
use crate::application::registration_service::RegistrationService;
use actix_web::{web, HttpResponse, Responder, ResponseError};

#[utoipa::path(
    post,
    path = "/api/v1/registration/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration created successfully"),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists")
    ),
    tag = "Registration"
)]
pub async fn register(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    match RegistrationService::register(state, req.into_inner()).await {
        Ok(res) => HttpResponse::Created().json(res),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/registration/resend",
    params(
        ("email" = String, Query, description = "Email to resend verification to")
    ),
    responses(
        (status = 200, description = "Verification email resent"),
        (status = 404, description = "Registration not found")
    ),
    tag = "Registration"
)]
pub async fn resend_verification(
    state: web::Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let email = query.get("email").cloned().unwrap_or_default();
    match RegistrationService::resend_verification(state, email).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => err.error_response(),
    }
}

/// Configure registration routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/registration")
            .route("/register", web::post().to(register))
            .route("/resend", web::post().to(resend_verification)),
    );
}