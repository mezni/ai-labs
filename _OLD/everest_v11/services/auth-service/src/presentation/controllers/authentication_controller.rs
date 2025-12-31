use actix_web::{post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    application::dtos::authentication::{LoginRequest, RefreshRequest},
    AppState,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(logout)
        .service(refresh)
        .service(validate);
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 403, description = "Account not verified")
    )
)]
#[post("/auth/login")]
async fn login(
    state: web::Data<AppState>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    match state.auth_service.login(req.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Authentication",
    security(
        ("bearer_auth" = [])
    ),
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized")
    )
)]
#[post("/auth/logout")]
async fn logout(
    state: web::Data<AppState>,
    req: web::Json<RefreshRequest>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.auth_service.logout(&req.refresh_token).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Logged out successfully"
        })),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "Authentication",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = LoginResponse),
        (status = 401, description = "Invalid refresh token")
    )
)]
#[post("/auth/refresh")]
async fn refresh(
    state: web::Data<AppState>,
    req: web::Json<RefreshRequest>,
) -> HttpResponse {
    match state.auth_service.refresh_token(&req.refresh_token).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/auth/validate",
    tag = "Authentication",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Token is valid", body = ValidateResponse),
        (status = 401, description = "Invalid or expired token")
    )
)]
#[post("/auth/validate")]
async fn validate(
    state: web::Data<AppState>,
    auth: BearerAuth,
) -> HttpResponse {
    match state.auth_service.validate_token(auth.token()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e.error_response(),
    }
}