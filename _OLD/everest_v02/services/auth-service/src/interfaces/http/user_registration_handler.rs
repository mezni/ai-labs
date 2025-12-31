use crate::AppState;
use crate::application::user_registration_dto::{
    RegisterUserRequest, RegisterUserResponse, VerifyRegistrationRequest,
    VerifyRegistrationResponse,
};
use crate::application::user_registration_service::UserRegistrationService;
use crate::core::errors::AppError;
use actix_web::{HttpResponse, web};

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "Registration created successfully", body = RegisterUserResponse),
        (status = 400, description = "Bad Request"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal error")
    ),
    tag = "Registration"
)]
pub async fn register_user(
    state: web::Data<AppState>,
    req: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, AppError> {
    if req.email.is_empty() || req.username.is_empty() {
        return Err(AppError::BadRequest(
            "Email and username are required".to_string(),
        ));
    }

    let response = UserRegistrationService::execute(&state, req.into_inner()).await?;
    Ok(HttpResponse::Created().json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/verify",
    request_body = VerifyRegistrationRequest,
    responses(
        (status = 200, description = "Email verified successfully", body = VerifyRegistrationResponse),
        (status = 400, description = "Invalid or expired token"),
        (status = 404, description = "Token not found"),
        (status = 500, description = "Internal error")
    ),
    tag = "Registration"
)]
pub async fn verify_user(
    state: web::Data<AppState>,
    req: web::Json<VerifyRegistrationRequest>,
) -> Result<HttpResponse, AppError> {
    if req.token.is_empty() {
        return Err(AppError::BadRequest("Token is required".to_string()));
    }

    let response = UserRegistrationService::verify(&state, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}
