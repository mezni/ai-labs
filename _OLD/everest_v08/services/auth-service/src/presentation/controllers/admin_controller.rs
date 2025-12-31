use crate::AppState;
use crate::application::admin_service::AdminService;
use actix_web::{HttpResponse, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/users")
            .route("", web::get().to(list_users))
            .route("/{id}", web::get().to(get_user))
            .route("", web::post().to(create_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}

#[utoipa::path(get, path = "/api/v1/admin/users", responses((status = 200, body = String)), tag = "Admin")]
async fn list_users(state: web::Data<AppState>) -> impl Responder {
    let svc = AdminService::new(state.into_inner());
    HttpResponse::Ok().body(svc.list_users().await)
}

#[utoipa::path(get, path = "/api/v1/admin/users/{id}", responses((status = 200, body = String)), tag = "Admin")]
async fn get_user(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let svc = AdminService::new(state.into_inner());
    HttpResponse::Ok().body(svc.get_user(id.into_inner()).await)
}

#[utoipa::path(post, path = "/api/v1/admin/users", responses((status = 201, body = String)), tag = "Admin")]
async fn create_user(state: web::Data<AppState>) -> impl Responder {
    let svc = AdminService::new(state.into_inner());
    HttpResponse::Created().body(svc.create_user().await)
}

#[utoipa::path(put, path = "/api/v1/admin/users/{id}", responses((status = 200, body = String)), tag = "Admin")]
async fn update_user(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let svc = AdminService::new(state.into_inner());
    HttpResponse::Ok().body(svc.update_user(id.into_inner()).await)
}

#[utoipa::path(delete, path = "/api/v1/admin/users/{id}", responses((status = 204)), tag = "Admin")]
async fn delete_user(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let svc = AdminService::new(state.into_inner());
    HttpResponse::NoContent().body(svc.delete_user(id.into_inner()).await)
}
