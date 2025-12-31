use crate::{
    application::dto::{ConnectorResponse, CreateConnectorRequest, UpdateConnectorRequest},
    domain::repositories::ConnectorRepositoryTrait,
    infrastructure::{
        ConnectorRepository,
        error::{AppError, AppResult},
    },
    utils::id_generator::generate_connector_id,
};

pub struct ConnectorService {
    repository: ConnectorRepository,
}

impl ConnectorService {
    pub fn new(repository: ConnectorRepository) -> Self {
        Self { repository }
    }

    pub async fn create_connector(
        &self,
        req: CreateConnectorRequest,
        created_by: String,
    ) -> AppResult<ConnectorResponse> {
        // Validate count constraints
        let count_available = req.count_available.unwrap_or(1);
        let count_total = req.count_total.unwrap_or(1);

        if count_available < 0 {
            return Err(AppError::ValidationError(
                "count_available must be >= 0".to_string(),
            ));
        }

        if count_total < 1 || count_total < count_available {
            return Err(AppError::ValidationError(
                "count_total must be >= 1 and >= count_available".to_string(),
            ));
        }

        let connector_id = generate_connector_id();

        let connector = self
            .repository
            .create(
                connector_id,
                req.station_id,
                req.connector_type_id,
                req.status_id,
                req.current_type_id,
                req.power_kw,
                req.voltage,
                req.amperage,
                Some(count_available),
                Some(count_total),
                created_by,
            )
            .await?;

        Ok(ConnectorResponse {
            connector_id: connector.connector_id,
            station_id: connector.station_id,
            connector_type_id: connector.connector_type_id,
            status_id: connector.status_id,
            current_type_id: connector.current_type_id,
            power_kw: connector.power_kw.map(|d| d.to_string()),
            voltage: connector.voltage,
            amperage: connector.amperage,
            count_available: connector.count_available.unwrap_or(1),
            count_total: connector.count_total.unwrap_or(1),
            created_at: connector.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: connector.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: connector.created_by,
            updated_by: connector.updated_by,
        })
    }

    pub async fn get_connector(&self, connector_id: &str) -> AppResult<ConnectorResponse> {
        let connector = self
            .repository
            .find_by_id(connector_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Connector {} not found", connector_id)))?;

        Ok(ConnectorResponse {
            connector_id: connector.connector_id,
            station_id: connector.station_id,
            connector_type_id: connector.connector_type_id,
            status_id: connector.status_id,
            current_type_id: connector.current_type_id,
            power_kw: connector.power_kw.map(|d| d.to_string()),
            voltage: connector.voltage,
            amperage: connector.amperage,
            count_available: connector.count_available.unwrap_or(1),
            count_total: connector.count_total.unwrap_or(1),
            created_at: connector.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: connector.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: connector.created_by,
            updated_by: connector.updated_by,
        })
    }

    pub async fn list_connectors_by_station(
        &self,
        station_id: &str,
    ) -> AppResult<Vec<ConnectorResponse>> {
        let connectors = self.repository.find_by_station(station_id).await?;

        Ok(connectors
            .into_iter()
            .map(|c| ConnectorResponse {
                connector_id: c.connector_id,
                station_id: c.station_id,
                connector_type_id: c.connector_type_id,
                status_id: c.status_id,
                current_type_id: c.current_type_id,
                power_kw: c.power_kw.map(|d| d.to_string()),
                voltage: c.voltage,
                amperage: c.amperage,
                count_available: c.count_available.unwrap_or(1),
                count_total: c.count_total.unwrap_or(1),
                created_at: c.created_at.map(|dt| dt.to_rfc3339()),
                updated_at: c.updated_at.map(|dt| dt.to_rfc3339()),
                created_by: c.created_by,
                updated_by: c.updated_by,
            })
            .collect())
    }

    pub async fn update_connector(
        &self,
        connector_id: &str,
        req: UpdateConnectorRequest,
        updated_by: String,
    ) -> AppResult<ConnectorResponse> {
        // Validate if both counts provided
        if let (Some(avail), Some(total)) = (req.count_available, req.count_total) {
            if avail < 0 || total < 1 || total < avail {
                return Err(AppError::ValidationError(
                    "Invalid count values".to_string(),
                ));
            }
        }

        let connector = self
            .repository
            .update(
                connector_id,
                req.status_id,
                req.power_kw,
                req.voltage,
                req.amperage,
                req.count_available,
                req.count_total,
                updated_by,
            )
            .await?;

        Ok(ConnectorResponse {
            connector_id: connector.connector_id,
            station_id: connector.station_id,
            connector_type_id: connector.connector_type_id,
            status_id: connector.status_id,
            current_type_id: connector.current_type_id,
            power_kw: connector.power_kw.map(|d| d.to_string()),
            voltage: connector.voltage,
            amperage: connector.amperage,
            count_available: connector.count_available.unwrap_or(1),
            count_total: connector.count_total.unwrap_or(1),
            created_at: connector.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: connector.updated_at.map(|dt| dt.to_rfc3339()),
            created_by: connector.created_by,
            updated_by: connector.updated_by,
        })
    }

    pub async fn delete_connector(&self, connector_id: &str) -> AppResult<()> {
        self.repository.delete(connector_id).await
    }
}
