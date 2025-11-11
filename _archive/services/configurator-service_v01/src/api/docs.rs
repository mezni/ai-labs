use utoipa::OpenApi;

use crate::api::handlers::network_handlers::{
    CreateNetworkRequest, ListNetworksQuery, NetworkResponse, UpdateNetworkRequest, create_network,
    delete_network, get_network, list_networks, update_network,
};
use utoipa_swagger_ui::SwaggerUi;

/// Define the OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        create_network,
        update_network,
        delete_network,
        get_network,
        list_networks,
    ),
    components(
        schemas(
            CreateNetworkRequest,
            UpdateNetworkRequest,
            NetworkResponse,
            ErrorResponse,
        )
    ),
    tags(
        (name = "networks", description = "Network management")
    )
)]
pub struct ApiDoc;

/// Configure Swagger UI route for Actix Web
pub fn configure_docs(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()),
    );
}
