use utoipa::OpenApi;
use crate::application::dtos::network_dtos::{CreateNetworkDto, UpdateNetworkDto, NetworkResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::network_handler::create_network,
        crate::api::handlers::network_handler::update_network,
    ),
    components(
        schemas(CreateNetworkDto, UpdateNetworkDto, NetworkResponse)
    ),
    tags(
        (name = "Network", description = "Network management endpoints")
    )
)]
pub struct ApiDoc;
