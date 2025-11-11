use crate::application::dtos::{CreateNetworkDto, NetworkQueryDto, UpdateNetworkDto};
use crate::application::services::{NetworkBusinessOperations, NetworkServiceTrait};
use crate::core::AppError;
use actix_web::{HttpResponse, delete, get, post, put, web};
use tracing::{error, info};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_network,
        get_network_by_id,
        get_network_by_name,
        list_networks,
        update_network,
        delete_network,
        get_network_statistics
    ),
    components(schemas(
        CreateNetworkDto,
        UpdateNetworkDto,
        NetworkQueryDto,
        crate::application::dtos::NetworkResponseDto,
        crate::application::dtos::NetworkListDto,
        crate::application::services::NetworkStatistics,
        AppError
    )),
    tags(
        (name = "Networks", description = "Network management endpoints")
    )
)]
pub struct NetworkApiDoc;

#[utoipa::path(
    post,
    path = "/networks",
    tag = "Networks",
    request_body = CreateNetworkDto,
    responses(
        (status = 201, description = "Network created successfully", body = NetworkResponseDto),
        (status = 400, description = "Invalid input data", body = AppError),
        (status = 409, description = "Network with same name already exists", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    )
)]
#[post("/networks")]
pub async fn create_network(
    service: web::Data<dyn NetworkServiceTrait>,
    dto: web::Json<CreateNetworkDto>,
) -> Result<HttpResponse, AppError> {
    info!("Creating new network: {}", dto.name);

    let network = service.create_network(dto.into_inner()).await?;

    Ok(HttpResponse::Created().json(network))
}

#[utoipa::path(
    get,
    path = "/networks/{id}",
    tag = "Networks",
    params(
        ("id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Network found", body = NetworkResponseDto),
        (status = 404, description = "Network not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    )
)]
#[get("/networks/{id}")]
pub async fn get_network_by_id(
    service: web::Data<dyn NetworkServiceTrait>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Getting network by ID: {}", id);

    match service.get_network_by_id(id).await? {
        Some(network) => Ok(HttpResponse::Ok().json(network)),
        None => {
            error!("Network not found with ID: {}", id);
            Err(AppError::not_found(&format!("Network with ID {}", id)))
        }
    }
}

#[utoipa::path(
    get,
    path = "/networks/name/{name}",
    tag = "Networks",
    params(
        ("name" = String, Path, description = "Network name")
    ),
    responses(
        (status = 200, description = "Network found", body = NetworkResponseDto),
        (status = 404, description = "Network not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    )
)]
#[get("/networks/name/{name}")]
pub async fn get_network_by_name(
    service: web::Data<dyn NetworkServiceTrait>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let name = path.into_inner();
    info!("Getting network by name: {}", name);

    match service.get_network_by_name(&name).await? {
        Some(network) => Ok(HttpResponse::Ok().json(network)),
        None => {
            error!("Network not found with name: {}", name);
            Err(AppError::not_found(&format!("Network with name {}", name)))
        }
    }
}

#[utoipa::path(
    get,
    path = "/networks",
    tag = "Networks",
    params(
        ("network_type" = Option<String>, Query, description = "Filter by network type"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order (asc/desc)")
    ),
    responses(
        (status = 200, description = "Networks retrieved successfully", body = NetworkListDto),
        (status = 500, description = "Internal server error", body = AppError)
    )
)]
#[get("/networks")]
pub async fn list_networks(
    service: web::Data<dyn NetworkServiceTrait>,
    query: web::Query<NetworkQueryDto>,
) -> Result<HttpResponse, AppError> {
    info!("Listing networks with query: {:?}", query);

    let networks = service.list_networks(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(networks))
}

#[utoipa::path(
    put,
    path = "/networks/{id}",
    tag = "Networks",
    params(
        ("id" = i32, Path, description = "Network ID")
    ),
    request_body = UpdateNetworkDto,
    responses(
        (status = 200, description = "Network updated successfully", body = NetworkResponseDto),
        (status = 400, description = "Invalid input data", body = AppError),
        (status = 404, description = "Network not found", body = AppError),
        (status = 409, description = "Network with same name already exists", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    )
)]
#[put("/networks/{id}")]
pub async fn update_network(
    service: web::Data<dyn NetworkServiceTrait>,
    path: web::Path<i32>,
    dto: web::Json<UpdateNetworkDto>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Updating network with ID: {}", id);

    let network = service.update_network(id, dto.into_inner()).await?;
    Ok(HttpResponse::Ok().json(network))
}

#[utoipa::path(
    delete,
    path = "/networks/{id}",
    tag = "Networks",
    params(
        ("id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 204, description = "Network deleted successfully"),
        (status = 404, description = "Network not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    )
)]
#[delete("/networks/{id}")]
pub async fn delete_network(
    service: web::Data<dyn NetworkServiceTrait>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Deleting network with ID: {}", id);

    service.delete_network(id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    get,
    path = "/networks/statistics",
    tag = "Networks",
    responses(
        (status = 200, description = "Statistics retrieved successfully", body = NetworkStatistics),
        (status = 500, description = "Internal server error", body = AppError)
    )
)]
#[get("/networks/statistics")]
pub async fn get_network_statistics(
    service: web::Data<dyn NetworkServiceTrait>,
) -> Result<HttpResponse, AppError> {
    info!("Getting network statistics");

    let stats = service.get_network_statistics().await?;
    Ok(HttpResponse::Ok().json(stats))
}
