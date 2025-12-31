use crate::application::dtos::admin::*;
use crate::core::errors::AppError;
use crate::domain::services::{AdminService as AdminServiceTrait, CreateUserData, UpdateUserData};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Debug, Deserialize, IntoParams)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/users",
    params(PaginationQuery),
    responses(
        (status = 200, description = "List of users", body = UserListResponse),
        (status = 403, description = "Forbidden")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
#[get("")]
pub async fn list_users(
    service: web::Data<Arc<dyn AdminServiceTrait>>,
    query: web::Query<PaginationQuery>,
) -> Result<impl Responder, AppError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let users = service.list_users(limit, offset).await?;
    let total = users.len();

    Ok(HttpResponse::Ok().json(UserListResponse {
        total,
        users: users.into_iter().map(UserResponse::from).collect(),
        limit,
        offset,
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
#[get("/{id}")]
pub async fn get_user(
    service: web::Data<Arc<dyn AdminServiceTrait>>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let user = service.get_user(&path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
#[post("")]
pub async fn create_user(
    service: web::Data<Arc<dyn AdminServiceTrait>>,
    body: web::Json<CreateUserRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let data = CreateUserData {
        email: body.email.clone(),
        username: body.username.clone(),
        password: body.password.clone(),
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        phone: body.phone.clone(),
        role: body.role.clone(),
    };

    let user = service.create_user(data).await?;
    Ok(HttpResponse::Created().json(UserResponse::from(user)))
}

#[utoipa::path(
    put,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 404, description = "User not found")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
#[put("/{id}")]
pub async fn update_user(
    service: web::Data<Arc<dyn AdminServiceTrait>>,
    path: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let data = UpdateUserData {
        email: body.email.clone(),
        username: body.username.clone(),
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        phone: body.phone.clone(),
        role: body.role.clone(),
        is_active: body.is_active,
    };

    let user = service.update_user(&path.into_inner(), data).await?;
    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
#[delete("/{id}")]
pub async fn delete_user(
    service: web::Data<Arc<dyn AdminServiceTrait>>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    service.delete_user(&path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User deleted successfully"
    })))
}
