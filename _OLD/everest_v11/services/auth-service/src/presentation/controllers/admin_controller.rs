use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use uuid::Uuid;

use crate::{
    application::dtos::admin::{CreateUserRequest, UpdateUserRequest},
    AppState,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user);
}

#[utoipa::path(
    get,
    path = "/admin/users",
    tag = "Admin",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of users", body = Vec<UserResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required")
    )
)]
#[get("/admin/users")]
async fn list_users(
    state: web::Data<AppState>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.admin_service.list_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/admin/users/{id}",
    tag = "Admin",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found")
    )
)]
#[get("/admin/users/{id}")]
async fn get_user(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.admin_service.get_user(path.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/admin/users",
    tag = "Admin",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "User already exists")
    )
)]
#[post("/admin/users")]
async fn create_user(
    state: web::Data<AppState>,
    req: web::Json<CreateUserRequest>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.admin_service.create_user(req.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    put,
    path = "/admin/users/{id}",
    tag = "Admin",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found")
    )
)]
#[put("/admin/users/{id}")]
async fn update_user(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateUserRequest>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.admin_service
        .update_user(path.into_inner(), req.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/admin/users/{id}",
    tag = "Admin",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found")
    )
)]
#[delete("/admin/users/{id}")]
async fn delete_user(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.admin_service.delete_user(path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User deleted successfully"
        })),
        Err(e) => e.error_response(),
    }
}