use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::application::health::{HealthResponse, VersionResponse};
use crate::interfaces::http::health;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        health::metrics,
        health::version
    ),
    components(
        schemas(HealthResponse, VersionResponse)
    ),
    tags(
        (name = "Health & Status", description = "Monitoring endpoints")
    )
)]
pub struct ApiDoc;

pub fn configure_swagger(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
