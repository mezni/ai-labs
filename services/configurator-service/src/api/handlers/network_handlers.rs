use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa::ToSchema;

use crate::application::{
    CreateNetworkRequest, NetworkApplicationService, NetworkResponse, UpdateNetworkRequest,
};
use shared::core::{ApiResponse, AppError, ErrorResponse};

/// Create Network
#[utoipa::path(
    post,
    path = "/api/v1/networks",
    request_body = CreateNetworkRequest,
    responses(
        (status = 201, description = "Network created", body = NetworkResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "networks"
)]
pub async fn create_network(
    service: web::Data<NetworkApplicationService>,
    req: web::Json<CreateNetworkRequest>,
) -> impl Responder {
    match service.create_network(req.into_inner()).await {
        Ok(network) => HttpResponse::Created().json(ApiResponse::success(network)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e.to_response())),
    }
}

/// Update Network
#[utoipa::path(
    put,
    path = "/api/v1/networks/{id}",
    request_body = UpdateNetworkRequest,
    params(
        ("id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Network updated", body = NetworkResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 404, description = "Network not found", body = ErrorResponse)
    ),
    tag = "networks"
)]
pub async fn update_network(
    service: web::Data<NetworkApplicationService>,
    path: web::Path<i32>,
    req: web::Json<UpdateNetworkRequest>,
) -> impl Responder {
    let command = crate::application::commands::UpdateNetworkCommand {
        id: path.into_inner(),
        request: req.into_inner(),
    };

    match service.update_network(command).await {
        Ok(network) => HttpResponse::Ok().json(ApiResponse::success(network)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().json(ApiResponse::<()>::error(&e.to_response()))
            }
            _ => HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e.to_response())),
        },
    }
}

/// Delete Network
#[utoipa::path(
    delete,
    path = "/api/v1/networks/{id}",
    params(
        ("id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 204, description = "Network deleted"),
        (status = 404, description = "Network not found", body = ErrorResponse)
    ),
    tag = "networks"
)]
pub async fn delete_network(
    service: web::Data<NetworkApplicationService>,
    path: web::Path<i32>,
) -> impl Responder {
    let command = crate::application::commands::DeleteNetworkCommand {
        id: path.into_inner(),
        deleted_by: "system".to_string(), // Replace with authenticated user if available
    };

    match service.delete_network(command).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().json(ApiResponse::<()>::error(&e.to_response()))
            }
            _ => HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e.to_response())),
        },
    }
}

/// Get Network by ID
#[utoipa::path(
    get,
    path = "/api/v1/networks/{id}",
    params(
        ("id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Network found", body = NetworkResponse),
        (status = 404, description = "Network not found", body = ErrorResponse)
    ),
    tag = "networks"
)]
pub async fn get_network(
    service: web::Data<NetworkApplicationService>,
    path: web::Path<i32>,
) -> impl Responder {
    let query = crate::application::queries::GetNetworkQuery {
        id: path.into_inner(),
    };

    match service.get_network(query).await {
        Ok(network) => HttpResponse::Ok().json(ApiResponse::success(network)),
        Err(e) => match e {
            AppError::NotFound(_) => {
                HttpResponse::NotFound().json(ApiResponse::<()>::error(&e.to_response()))
            }
            _ => HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e.to_response())),
        },
    }
}

/// List Networks
#[utoipa::path(
    get,
    path = "/api/v1/networks",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "List of networks", body = [NetworkResponse])
    ),
    tag = "networks"
)]
pub async fn list_networks(
    service: web::Data<NetworkApplicationService>,
    query: web::Query<ListNetworksQuery>,
) -> impl Responder {
    let query_struct = crate::application::queries::ListNetworksQuery {
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(20),
    };

    match service.list_networks(query_struct).await {
        Ok(networks) => HttpResponse::Ok().json(ApiResponse::success(networks)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(&e.to_response())),
    }
}

/// Query struct for list endpoint
#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ListNetworksQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
