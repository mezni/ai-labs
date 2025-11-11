use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health_handlers::health_check,
        crate::api::handlers::health_handlers::database_health,
        crate::api::handlers::network_handlers::create_network,
        crate::api::handlers::network_handlers::get_network_by_id,
        crate::api::handlers::network_handlers::get_network_by_name,
        crate::api::handlers::network_handlers::list_networks,
        crate::api::handlers::network_handlers::update_network,
        crate::api::handlers::network_handlers::delete_network,
        crate::api::handlers::network_handlers::get_network_statistics
    ),
    components(schemas(
        crate::application::dtos::CreateNetworkDto,
        crate::application::dtos::UpdateNetworkDto,
        crate::application::dtos::NetworkResponseDto,
        crate::application::dtos::NetworkListDto,
        crate::application::dtos::NetworkQueryDto,
        crate::application::services::NetworkStatistics,
        crate::api::handlers::health_handlers::HealthResponse,
        crate::core::AppError
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Networks", description = "Network management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_token",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}

pub fn configure_openapi() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

/// Helper function to get OpenAPI JSON
pub fn get_openapi_json() -> String {
    ApiDoc::openapi().to_json().unwrap()
}
