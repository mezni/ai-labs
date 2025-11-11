use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health_handlers::health_check,
        crate::api::handlers::health_handlers::database_health,
        crate::api::handlers::network_handlers::get_network,
        crate::api::handlers::network_handlers::create_network
    ),
    components(
        schemas(
            crate::api::handlers::health_handlers::HealthResponse,
            crate::api::handlers::network_handlers::NetworkResponse
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Network", description = "Network endpoints")
    )
)]
pub struct ApiDoc;
