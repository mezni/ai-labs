use actix_web::{HttpResponse, get, post, web};
use serde::Deserialize;
use utoipa::ToSchema;

#[utoipa::path(
    get,
    path = "/api/v1/networks",
    tag = "Network",
    responses(
        (status = 200, description = "Get networks", body = [NetworkResponse])
    )
)]
#[get("")] // Empty path since it's under /api/v1/networks scope
pub async fn get_network() -> HttpResponse {
    HttpResponse::Ok().json(Vec::<NetworkResponse>::new())
}

#[utoipa::path(
    post,
    path = "/api/v1/networks",
    tag = "Network",
    request_body = CreateNetworkRequest,
    responses(
        (status = 201, description = "Network created", body = NetworkResponse)
    )
)]
#[post("")] // Empty path since it's under /api/v1/networks scope
pub async fn create_network(body: web::Json<CreateNetworkRequest>) -> HttpResponse {
    HttpResponse::Created().json(NetworkResponse {
        id: 0,
        name: body.name.clone(),
    })
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct NetworkResponse {
    pub id: u32,
    pub name: String,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct CreateNetworkRequest {
    pub name: String,
}
