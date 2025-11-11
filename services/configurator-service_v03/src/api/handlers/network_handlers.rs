use actix_web::{HttpResponse, get, post, web};
use utoipa::ToSchema;

use crate::application::services::network_services::NetworkService;
use crate::domain::network::NetworkId;
use crate::application::dtos::network_dtos::{CreateNetworkDto, UpdateNetworkDto};

/// Response DTO for API
#[derive(serde::Serialize, ToSchema)]
pub struct NetworkResponse {
    pub id: i32,
    pub name: String,
}

/// GET /api/v1/networks
#[utoipa::path(
    get,
    path = "/api/v1/networks",
    tag = "Network",
    responses(
        (status = 200, description = "Get all networks", body = [NetworkResponse])
    )
)]
#[get("")]
pub async fn get_network(
    service: web::Data<NetworkService<impl crate::domain::network::NetworkRepository>>,
) -> HttpResponse {
    match service.list_networks().await {
        Ok(networks) => {
            let response: Vec<NetworkResponse> = networks
                .into_iter()
                .map(|n| NetworkResponse {
                    id: n.id().0,
                    name: n.name().to_string(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// POST /api/v1/networks
#[utoipa::path(
    post,
    path = "/api/v1/networks",
    tag = "Network",
    request_body = CreateNetworkDto,
    responses(
        (status = 201, description = "Network created", body = NetworkResponse)
    )
)]
#[post("")]
pub async fn create_network(
    service: web::Data<NetworkService<impl crate::domain::network::NetworkRepository>>,
    body: web::Json<CreateNetworkDto>,
) -> HttpResponse {
    match service.create_network(body.into_inner()).await {
        Ok(network) => HttpResponse::Created().json(NetworkResponse {
            id: network.id().0,
            name: network.name().to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
