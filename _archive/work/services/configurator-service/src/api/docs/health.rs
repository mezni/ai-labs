use utoipa::OpenApi;
use crate::core::database::PoolStats;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health::health_check,
        crate::api::handlers::health::pool_stats,
    ),
    components(schemas(PoolStats)),
    tags(
        (name = "Health", description = "Health check and monitoring endpoints")
    )
)]
pub struct HealthApiDoc;

impl HealthApiDoc {
    pub fn create_openapi() -> utoipa::openapi::OpenApi {
        Self::openapi()
    }
}