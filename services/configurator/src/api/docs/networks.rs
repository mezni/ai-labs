use utoipa::OpenApi;
use crate::domain::networks::Network;
use crate::application::dtos::networks::{NetworkCreate, NetworkUpdate};
use crate::core::database::PoolStats;

// Add security scheme to OpenAPI
pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            utoipa::openapi::security::SecurityScheme::Http(
                utoipa::openapi::security::HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health::health_check,
        crate::api::handlers::health::pool_stats,
        crate::api::handlers::networks::get_all_networks,
        crate::api::handlers::networks::get_network_by_id,
        crate::api::handlers::networks::create_network,
        crate::api::handlers::networks::update_network,
        crate::api::handlers::networks::delete_network
    ),
    components(schemas(Network, NetworkCreate, NetworkUpdate, PoolStats)),
    tags(
        (name = "Network", description = "Network management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn create_openapi() -> utoipa::openapi::OpenApi {
        Self::openapi()
    }
}