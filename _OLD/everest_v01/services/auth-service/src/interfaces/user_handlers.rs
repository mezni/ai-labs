// src/interfaces/user_handlers.rs
use crate::application::{
    ListUsersRequest, PaginatedUsersResponse, UpdateProfileRequest, UserDetailResponse,
    UserResponse, UserService,
};
use crate::core::{AppError, extract_claims};
use actix_web::{HttpRequest, HttpResponse, Responder, get, put, web};
use validator::Validate;

/// Get current user profile
#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    tag = "Users",
    responses(
        (status = 200, description = "User profile retrieved", body = UserResponse),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/me")]
pub async fn get_my_profile(
    user_service: web::Data<UserService>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    let response = user_service.queries.get_my_profile(&claims.user_id).await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Update current user profile
#[utoipa::path(
    put,
    path = "/api/v1/users/me",
    tag = "Users",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UserResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/me")]
pub async fn update_my_profile(
    user_service: web::Data<UserService>,
    request: web::Json<UpdateProfileRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let response = user_service
        .commands
        .update_profile(claims.user_id, request.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

/// List users (admin only)
#[utoipa::path(
    get,
    path = "/api/v1/admin/users",
    tag = "Admin - Users",
    params(
        ("search" = Option<String>, Query, description = "Search by email, username, or name"),
        ("role" = Option<String>, Query, description = "Filter by role"),
        ("source" = Option<String>, Query, description = "Filter by source (web/internal)"),
        ("network_id" = Option<String>, Query, description = "Filter by network ID"),
        ("station_id" = Option<String>, Query, description = "Filter by station ID"),
        ("is_active" = Option<bool>, Query, description = "Filter by active status"),
        ("is_verified" = Option<bool>, Query, description = "Filter by verification status"),
        ("include_deleted" = Option<bool>, Query, description = "Include deleted users"),
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("page_size" = Option<i64>, Query, description = "Page size (default: 20, max: 100)"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order (asc/desc)")
    ),
    responses(
        (status = 200, description = "Users retrieved successfully", body = PaginatedUsersResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("")]
pub async fn list_users(
    user_service: web::Data<UserService>,
    query: web::Query<ListUsersRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    // Check if user is admin
    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can list users".to_string(),
        ));
    }

    query
        .validate()
        .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

    let response = user_service.queries.list_users(query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Get user by ID (admin only)
#[utoipa::path(
    get,
    path = "/api/v1/admin/users/{user_id}",
    tag = "Admin - Users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details retrieved", body = UserDetailResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/{user_id}")]
pub async fn get_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<String>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = extract_claims(&http_req)?;

    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can view user details".to_string(),
        ));
    }

    let response = user_service
        .queries
        .get_user_detail(&user_id.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(response))
}
