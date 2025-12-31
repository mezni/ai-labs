use crate::AppState;
use crate::application::authentication_service::AuthenticationService;
use crate::application::dtos::authentication::{AuthResponse, LoginRequest};
use actix_web::{HttpResponse, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh))
            .route("/validate", web::get().to(validate)), // Changed to GET as it's common for validation
    );
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses((status = 200, description = "Login successful", body = AuthResponse)),
    tag = "Authentication"
)]
async fn login(state: web::Data<AppState>) -> impl Responder {
    let svc = AuthenticationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.login().await)
}

#[utoipa::path(post, path = "/api/v1/auth/logout", responses((status = 200)), tag = "Authentication")]
async fn logout(state: web::Data<AppState>) -> impl Responder {
    let svc = AuthenticationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.logout().await)
}

#[utoipa::path(post, path = "/api/v1/auth/refresh", responses((status = 200, body = AuthResponse)), tag = "Authentication")]
async fn refresh(state: web::Data<AppState>) -> impl Responder {
    let svc = AuthenticationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.refresh().await)
}

#[utoipa::path(get, path = "/api/v1/auth/validate", responses((status = 200)), tag = "Authentication")]
async fn validate(state: web::Data<AppState>) -> impl Responder {
    let svc = AuthenticationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.validate().await)
}
