use crate::{
    application::dto::{CreateStationRequest, StationResponse, UpdateStationRequest},
    domain::repositories::StationRepositoryTrait,
    infrastructure::{StationRepository, error::AppResult},
    utils::id_generator::generate_station_id,
};

pub struct StationService {
    repository: StationRepository,
}

impl StationService {
    pub fn new(repository: StationRepository) -> Self {
        Self { repository }
    }

    pub async fn create_station(
        &self,
        req: CreateStationRequest,
        created_by: String,
    ) -> AppResult<StationResponse> {
        let station_id = generate_station_id();

        let station = self
            .repository
            .create(
                station_id,
                req.osm_id,
                req.name,
                req.address,
                req.latitude,
                req.longitude,
                req.tags,
                req.network_id,
                created_by,
            )
            .await?;

        Ok(StationResponse {
            station_id: station.station_id,
            osm_id: station.osm_id,
            name: station.name,
            address: station.address,
            latitude: station.latitude,
            longitude: station.longitude,
            tags: station.tags,
            network_id: station.network_id,
            created_at: station.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: station.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: station.created_by,
            updated_by: station.updated_by,
        })
    }

    pub async fn get_station(&self, station_id: &str) -> AppResult<StationResponse> {
        let station = self
            .repository
            .find_by_id(station_id)
            .await?
            .ok_or_else(|| {
                crate::infrastructure::error::AppError::NotFound(format!(
                    "Station {} not found",
                    station_id
                ))
            })?;

        Ok(StationResponse {
            station_id: station.station_id,
            osm_id: station.osm_id,
            name: station.name,
            address: station.address,
            latitude: station.latitude,
            longitude: station.longitude,
            tags: station.tags,
            network_id: station.network_id,
            created_at: station.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: station.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: station.created_by,
            updated_by: station.updated_by,
        })
    }

    pub async fn list_stations(&self, page: i64, limit: i64) -> AppResult<Vec<StationResponse>> {
        let offset = (page - 1) * limit;
        let stations = self.repository.find_all(limit, offset).await?;

        Ok(stations
            .into_iter()
            .map(|s| StationResponse {
                station_id: s.station_id,
                osm_id: s.osm_id,
                name: s.name,
                address: s.address,
                latitude: s.latitude,
                longitude: s.longitude,
                tags: s.tags,
                network_id: s.network_id,
                created_at: s.created_at.map(|dt| dt.to_rfc3339()),
                updated_at: s.updated_at.map(|dt| dt.to_rfc3339()),
                created_by: s.created_by,
                updated_by: s.updated_by,
            })
            .collect())
    }

    pub async fn list_stations_by_network(
        &self,
        network_id: &str,
    ) -> AppResult<Vec<StationResponse>> {
        let stations = self.repository.find_by_network(network_id).await?;

        Ok(stations
            .into_iter()
            .map(|s| StationResponse {
                station_id: s.station_id,
                osm_id: s.osm_id,
                name: s.name,
                address: s.address,
                latitude: s.latitude,
                longitude: s.longitude,
                tags: s.tags,
                network_id: s.network_id,
                created_at: s.created_at.map(|dt| dt.to_rfc3339()),
                updated_at: s.updated_at.map(|dt| dt.to_rfc3339()),
                created_by: s.created_by,
                updated_by: s.updated_by,
            })
            .collect())
    }

    pub async fn update_station(
        &self,
        station_id: &str,
        req: UpdateStationRequest,
        updated_by: String,
    ) -> AppResult<StationResponse> {
        let station = self
            .repository
            .update(
                station_id,
                req.name,
                req.address,
                req.latitude,
                req.longitude,
                req.tags,
                req.network_id,
                updated_by,
            )
            .await?;

        Ok(StationResponse {
            station_id: station.station_id,
            osm_id: station.osm_id,
            name: station.name,
            address: station.address,
            latitude: station.latitude,
            longitude: station.longitude,
            tags: station.tags,
            network_id: station.network_id,
            created_at: station.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: station.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: station.created_by,
            updated_by: station.updated_by,
        })
    }

    pub async fn delete_station(&self, station_id: &str) -> AppResult<()> {
        self.repository.delete(station_id).await
    }
}
