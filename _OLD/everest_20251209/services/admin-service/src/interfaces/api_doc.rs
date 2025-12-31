use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::application::dto::*;
use crate::domain::entities::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::handlers::create_network,
        crate::interfaces::handlers::get_network,
        crate::interfaces::handlers::list_networks,
        crate::interfaces::handlers::update_network,
        crate::interfaces::handlers::delete_network,
        crate::interfaces::handlers::create_station,
        crate::interfaces::handlers::get_station,
        crate::interfaces::handlers::list_stations,
        crate::interfaces::handlers::list_stations_by_network,
        crate::interfaces::handlers::update_station,
        crate::interfaces::handlers::delete_station,
        crate::interfaces::handlers::create_connector,
        crate::interfaces::handlers::get_connector,
        crate::interfaces::handlers::list_connectors_by_station,
        crate::interfaces::handlers::update_connector,
        crate::interfaces::handlers::delete_connector,
        crate::interfaces::handlers::health_check,
    ),
    components(
        schemas(
            NetworkType,
            Network,
            Station,
            Connector,
            CreateNetworkRequest,
            UpdateNetworkRequest,
            NetworkResponse,
            CreateStationRequest,
            UpdateStationRequest,
            StationResponse,
            CreateConnectorRequest,
            UpdateConnectorRequest,
            ConnectorResponse,
            PaginationParams,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "networks", description = "Network management endpoints"),
        (name = "stations", description = "Station management endpoints"),
        (name = "connectors", description = "Connector management endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Admin Service API",
        version = "1.0.0",
        description = "EV Charging Network Administration Service with JWT Authentication (requires 'admin' role)",
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}