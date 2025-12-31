use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::application::dto::*;
use crate::domain::entities::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::handlers::get_nearby_stations,
        crate::interfaces::handlers::create_review,
        crate::interfaces::handlers::get_station_reviews,
        crate::interfaces::handlers::update_review,
        crate::interfaces::handlers::delete_review,
        crate::interfaces::handlers::health_check,
        crate::interfaces::handlers::get_user_info,
    ),
    components(
        schemas(
            NearbyStationsQuery,
            StationResponse,
            CreateReviewRequest,
            UpdateReviewRequest,
            ReviewResponse,
            Station,
            UserReview,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "stations", description = "Station management endpoints"),
        (name = "reviews", description = "Review management endpoints"),
        (name = "user", description = "User information endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Locate Service API",
        version = "1.0.0",
        description = "EV Charging Station Locator Service with JWT Authentication",
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
