use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthResponseDto {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = "up")]
    pub database: String,
    #[schema(example = "up")]
    pub keycloak: String,
}
