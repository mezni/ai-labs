use crate::domain::entities::{Connector, Network, Station};
use crate::infrastructure::error::AppResult;

#[async_trait::async_trait]
pub trait NetworkRepositoryTrait {
    async fn create(
        &self,
        network_id: String,
        name: String,
        network_type: String,
        support_phone: Option<String>,
        support_email: Option<String>,
        created_by: String,
    ) -> AppResult<Network>;

    async fn find_by_id(&self, network_id: &str) -> AppResult<Option<Network>>;

    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<Network>>;

    async fn update(
        &self,
        network_id: &str,
        name: Option<String>,
        network_type: Option<String>,
        support_phone: Option<String>,
        support_email: Option<String>,
        is_verified: Option<bool>,
        updated_by: String,
    ) -> AppResult<Network>;

    async fn delete(&self, network_id: &str) -> AppResult<()>;
}

#[async_trait::async_trait]
pub trait StationRepositoryTrait {
    async fn create(
        &self,
        station_id: String,
        osm_id: i64,
        name: String,
        address: Option<String>,
        latitude: f64,
        longitude: f64,
        tags: Option<serde_json::Value>,
        network_id: Option<String>,
        created_by: String,
    ) -> AppResult<Station>;

    async fn find_by_id(&self, station_id: &str) -> AppResult<Option<Station>>;

    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<Station>>;

    async fn find_by_network(&self, network_id: &str) -> AppResult<Vec<Station>>;

    async fn update(
        &self,
        station_id: &str,
        name: Option<String>,
        address: Option<String>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        tags: Option<serde_json::Value>,
        network_id: Option<String>,
        updated_by: String,
    ) -> AppResult<Station>;

    async fn delete(&self, station_id: &str) -> AppResult<()>;
}

#[async_trait::async_trait]
pub trait ConnectorRepositoryTrait {
    async fn create(
        &self,
        connector_id: String,
        station_id: String,
        connector_type_id: i64,
        status_id: i64,
        current_type_id: i64,
        power_kw: Option<f64>,
        voltage: Option<i32>,
        amperage: Option<i32>,
        count_available: Option<i32>,
        count_total: Option<i32>,
        created_by: String,
    ) -> AppResult<Connector>;

    async fn find_by_id(&self, connector_id: &str) -> AppResult<Option<Connector>>;

    async fn find_by_station(&self, station_id: &str) -> AppResult<Vec<Connector>>;

    async fn update(
        &self,
        connector_id: &str,
        status_id: Option<i64>,
        power_kw: Option<f64>,
        voltage: Option<i32>,
        amperage: Option<i32>,
        count_available: Option<i32>,
        count_total: Option<i32>,
        updated_by: String,
    ) -> AppResult<Connector>;

    async fn delete(&self, connector_id: &str) -> AppResult<()>;
}
