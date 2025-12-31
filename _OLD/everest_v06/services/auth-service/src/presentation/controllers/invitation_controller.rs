use crate::AppState;
use crate::application::invitation_service::InvitationService;
use actix_web::{HttpResponse, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/invitations")
            .route("", web::post().to(create))
            .route("", web::get().to(list))
            .route("/{code}", web::get().to(get))
            .route("/{code}/accept", web::post().to(accept))
            .route("/{code}", web::delete().to(cancel)),
    );
}

#[utoipa::path(post, path = "/api/v1/invitations", responses((status = 201)), tag = "Invitations")]
async fn create(state: web::Data<AppState>) -> impl Responder {
    let svc = InvitationService::new(state.into_inner());
    HttpResponse::Created().body(svc.create().await)
}

#[utoipa::path(get, path = "/api/v1/invitations", responses((status = 200)), tag = "Invitations")]
async fn list(state: web::Data<AppState>) -> impl Responder {
    let svc = InvitationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.list().await)
}

#[utoipa::path(get, path = "/api/v1/invitations/{code}", responses((status = 200)), tag = "Invitations")]
async fn get(state: web::Data<AppState>, code: web::Path<String>) -> impl Responder {
    let svc = InvitationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.get(code.into_inner()).await)
}

#[utoipa::path(post, path = "/api/v1/invitations/{code}/accept", responses((status = 200)), tag = "Invitations")]
async fn accept(state: web::Data<AppState>, code: web::Path<String>) -> impl Responder {
    let svc = InvitationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.accept(code.into_inner()).await)
}

#[utoipa::path(delete, path = "/api/v1/invitations/{code}", responses((status = 204)), tag = "Invitations")]
async fn cancel(state: web::Data<AppState>, code: web::Path<String>) -> impl Responder {
    let svc = InvitationService::new(state.into_inner());
    HttpResponse::NoContent().body(svc.cancel(code.into_inner()).await)
}
