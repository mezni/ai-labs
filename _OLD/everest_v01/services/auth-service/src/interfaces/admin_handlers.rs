// src/interfaces/admin_handlers.rs
use crate::application::{
    AdminResetPasswordRequest, AdminUpdateUserRequest, CreateInternalUserRequest,
    DeleteUserResponse, SuccessResponse, UserResponse, UserService,
};
use crate::core::{AppError, extract_claims};
use actix_web::{HttpRequest, HttpResponse, Responder, delete, post, put, web};
use validator::Validate;

/// Create internal user (admin only)
#[utoipa::path(
    post,
    path = "/api/v1/admin/users",
    tag = "Admin - Users",
    request_body = CreateInternalUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 409, description = "Email or username already exists")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("")]
pub async fn create_internal_user(
    user_service: web::Data<UserService>,
    request: web::Json<CreateInternalUserRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can create internal users".to_string(),
        ));
    }

    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let response = user_service
        .commands
        .create_internal_user(request.into_inner(), claims.user_id)
        .await?;

    Ok(HttpResponse::Created().json(response))
}

/// Update user (admin only)
#[utoipa::path(
    put,
    path = "/api/v1/admin/users/{user_id}",
    tag = "Admin - Users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = AdminUpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/{user_id}")]
pub async fn update_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<String>,
    request: web::Json<AdminUpdateUserRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can update users".to_string(),
        ));
    }

    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let response = user_service
        .commands
        .admin_update_user(user_id.into_inner(), request.into_inner(), claims.user_id)
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Delete user (admin only)
#[utoipa::path(
    delete,
    path = "/api/v1/admin/users/{user_id}",
    tag = "Admin - Users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = DeleteUserResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[delete("/{user_id}")]
pub async fn delete_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<String>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can delete users".to_string(),
        ));
    }

    let response = user_service
        .commands
        .delete_user(user_id.into_inner(), claims.user_id)
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Reset user password (admin only)
#[utoipa::path(
    post,
    path = "/api/v1/admin/users/{user_id}/reset-password",
    tag = "Admin - Users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = AdminResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successfully", body = SuccessResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/{user_id}/reset-password")]
pub async fn reset_password(
    user_service: web::Data<UserService>,
    user_id: web::Path<String>,
    request: web::Json<AdminResetPasswordRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can reset passwords".to_string(),
        ));
    }

    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    user_service
        .commands
        .admin_reset_password(user_id.into_inner(), request.into_inner(), claims.user_id)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        message: "Password reset successfully".to_string(),
    }))
}
