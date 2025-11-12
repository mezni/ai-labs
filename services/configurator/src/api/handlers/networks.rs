use actix_web::{web, HttpResponse};
use crate::application::services::networks::NetworkService;
use crate::application::dtos::networks::{NetworkCreate, NetworkUpdate};
use crate::core::errors::AppError;
use crate::domain::networks::Network;
use crate::core::middleware::{AuthenticatedUser, require_role};
use utoipa::path;

// Get all networks (public)
#[utoipa::path(
    get,
    path = "/api/v1/networks",
    responses(
        (status = 200, description = "List networks", body = [Network]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Network"
)]
pub async fn get_all_networks(
    service: web::Data<NetworkService>,
) -> Result<HttpResponse, AppError> {
    let networks = service.get_all().await?;
    Ok(HttpResponse::Ok().json(networks))
}

// Get network by ID (public)
#[utoipa::path(
    get,
    path = "/api/v1/networks/{network_id}",
    params(
        ("network_id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Get network by id", body = Network),
        (status = 404, description = "Network not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Network"
)]
pub async fn get_network_by_id(
    network_id: web::Path<i32>,
    service: web::Data<NetworkService>,
) -> Result<HttpResponse, AppError> {
    let network = service.get_by_id(network_id.into_inner()).await?;
    match network {
        Some(network) => Ok(HttpResponse::Ok().json(network)),
        None => Err(AppError::not_found("Network")),
    }
}

// Create network (protected - requires authentication)
#[utoipa::path(
    post,
    path = "/api/v1/networks",
    request_body = NetworkCreate,
    responses(
        (status = 201, description = "Network created", body = Network),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failed to create network")
    ),
    tag = "Network",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_network(
    network: web::Json<NetworkCreate>,
    service: web::Data<NetworkService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    if let Some(ref email) = network.contact_email {
        if !is_valid_email(email) {
            return Err(AppError::validation("Invalid email format"));
        }
    }

    let network = service.create(network.into_inner(), user.username).await?;
    Ok(HttpResponse::Created().json(network))
}

// Update network (protected - requires authentication)
#[utoipa::path(
    put,
    path = "/api/v1/networks/{network_id}",
    request_body = NetworkUpdate,
    params(
        ("network_id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Network updated", body = Network),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Network not found"),
        (status = 500, description = "Failed to update network")
    ),
    tag = "Network",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_network(
    network_id: web::Path<i32>,
    network: web::Json<NetworkUpdate>,
    service: web::Data<NetworkService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    if let Some(ref email) = network.contact_email {
        if !is_valid_email(email) {
            return Err(AppError::validation("Invalid email format"));
        }
    }

    let network = service
        .update(network_id.into_inner(), network.into_inner(), user.username)
        .await?;
    Ok(HttpResponse::Ok().json(network))
}

// Delete network (protected - requires admin role)
#[utoipa::path(
    delete,
    path = "/api/v1/networks/{network_id}",
    params(
        ("network_id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 204, description = "Network deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin only"),
        (status = 404, description = "Network not found"),
        (status = 500, description = "Failed to delete network")
    ),
    tag = "Network",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_network(
    network_id: web::Path<i32>,
    service: web::Data<NetworkService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    require_role(&user, "admin")
        .map_err(|e| AppError::authorization(&e.to_string()))?;

    service.delete(network_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Simple email validation function
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local_part = parts[0];
    let domain_part = parts[1];

    // Check local part is not empty
    if local_part.is_empty() {
        return false;
    }

    // Check domain part has at least one dot and valid structure
    let domain_parts: Vec<&str> = domain_part.split('.').collect();
    if domain_parts.len() < 2 {
        return false;
    }

    // Check each domain part is not empty
    for part in domain_parts {
        if part.is_empty() {
            return false;
        }
    }

    // Basic character check
    if email.contains(' ') || email.contains("..") {
        return false;
    }

    true
}