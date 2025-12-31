use crate::{
    application::dto::StationResponse,
    domain::repositories::StationRepositoryTrait,
    infrastructure::{StationRepository, error::AppResult},
};

pub struct StationService {
    repository: StationRepository,
}

impl StationService {
    pub fn new(repository: StationRepository) -> Self {
        Self { repository }
    }

    pub async fn find_nearby_stations(
        &self,
        latitude: f64,
        longitude: f64,
        radius_meters: i32,
        limit: i32,
    ) -> AppResult<Vec<StationResponse>> {
        let stations = self
            .repository
            .find_nearby(latitude, longitude, radius_meters, limit)
            .await?;

        let response: Vec<StationResponse> = stations
            .into_iter()
            .map(|s| StationResponse {
                station_id: s.station_id,
                name: s.name,
                address: s.address,
                distance_meters: s.distance_meters,
                has_available_connectors: s.has_available_connectors,
                total_available_connectors: s.total_available_connectors,
                max_power_kw: s.max_power_kw,
                power_tier: s.power_tier,
                operator: s.operator,
            })
            .collect();

        Ok(response)
    }
}
