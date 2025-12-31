use actix_web::{web, HttpRequest, HttpResponse};
use crate::{
    application::{
        dto::*,
        ConnectorService, NetworkService, StationService,
    },
    infrastructure::error::{AppError, AppResult},
    middleware::extract_claims,
};

// Network Handlers
#[utoipa::path(
    post,
    path = "/api/v1/networks",
    request_body = CreateNetworkRequest,
    responses(
        (status = 201, description = "Network created", body = NetworkResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin role required")
    ),
    security(("bearer_auth" = [])),
    tag = "networks"
)]
pub async fn create_network(
    req: HttpRequest,
    payload: web::Json<CreateNetworkRequest>,
    service: web::Data<NetworkService>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    let network = service.create_network(payload.into_inner(), claims.jti).await?;
    Ok(HttpResponse::Created().json(network))
}

#[utoipa::path(
    get,
    path = "/api/v1/networks/{network_id}",
    params(
        ("network_id" = String, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Network found", body = NetworkResponse),
        (status = 404, description = "Network not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "networks"
)]
pub async fn get_network(
    _req: HttpRequest,
    network_id: web::Path<String>,
    service: web::Data<NetworkService>,
) -> AppResult<HttpResponse> {
    let network = service.get_network(&network_id).await?;
    Ok(HttpResponse::Ok().json(network))
}

#[utoipa::path(
    get,
    path = "/api/v1/networks",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<i64>, Query, description = "Items per page (default: 50)")
    ),
    responses(
        (status = 200, description = "List of networks", body = Vec<NetworkResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "networks"
)]
pub async fn list_networks(
    _req: HttpRequest,
    query: web::Query<PaginationParams>,
    service: web::Data<NetworkService>,
) -> AppResult<HttpResponse> {
    let networks = service.list_networks(query.page, query.limit).await?;
    Ok(HttpResponse::Ok().json(networks))
}

#[utoipa::path(
    put,
    path = "/api/v1/networks/{network_id}",
    params(
        ("network_id" = String, Path, description = "Network ID")
    ),
    request_body = UpdateNetworkRequest,
    responses(
        (status = 200, description = "Network updated", body = NetworkResponse),
        (status = 404, description = "Network not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = [])),
    tag = "networks"
)]
pub async fn update_network(
    req: HttpRequest,
    network_id: web::Path<String>,
    payload: web::Json<UpdateNetworkRequest>,
    service: web::Data<NetworkService>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    let network = service
        .update_network(&network_id, payload.into_inner(), claims.jti)
        .await?;
    Ok(HttpResponse::Ok().json(network))
}

#[utoipa::path(
    delete,
    path = "/api/v1/networks/{network_id}",
    params(
        ("network_id" = String, Path, description = "Network ID")
    ),
    responses(
        (status = 204, description = "Network deleted"),
        (status = 404, description = "Network not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "networks"
)]
pub async fn delete_network(
    _req: HttpRequest,
    network_id: web::Path<String>,
    service: web::Data<NetworkService>,
) -> AppResult<HttpResponse> {
    service.delete_network(&network_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Station Handlers
#[utoipa::path(
    post,
    path = "/api/v1/stations",
    request_body = CreateStationRequest,
    responses(
        (status = 201, description = "Station created", body = StationResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "stations"
)]
pub async fn create_station(
    req: HttpRequest,
    payload: web::Json<CreateStationRequest>,
    service: web::Data<StationService>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    let station = service.create_station(payload.into_inner(), claims.jti).await?;
    Ok(HttpResponse::Created().json(station))
}

#[utoipa::path(
    get,
    path = "/api/v1/stations/{station_id}",
    params(
        ("station_id" = String, Path, description = "Station ID")
    ),
    responses(
        (status = 200, description = "Station found", body = StationResponse),
        (status = 404, description = "Station not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "stations"
)]
pub async fn get_station(
    _req: HttpRequest,
    station_id: web::Path<String>,
    service: web::Data<StationService>,
) -> AppResult<HttpResponse> {
    let station = service.get_station(&station_id).await?;
    Ok(HttpResponse::Ok().json(station))
}

#[utoipa::path(
    get,
    path = "/api/v1/stations",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of stations", body = Vec<StationResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "stations"
)]
pub async fn list_stations(
    _req: HttpRequest,
    query: web::Query<PaginationParams>,
    service: web::Data<StationService>,
) -> AppResult<HttpResponse> {
    let stations = service.list_stations(query.page, query.limit).await?;
    Ok(HttpResponse::Ok().json(stations))
}

#[utoipa::path(
    get,
    path = "/api/v1/networks/{network_id}/stations",
    params(
        ("network_id" = String, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "List of stations", body = Vec<StationResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "stations"
)]
pub async fn list_stations_by_network(
    _req: HttpRequest,
    network_id: web::Path<String>,
    service: web::Data<StationService>,
) -> AppResult<HttpResponse> {
    let stations = service.list_stations_by_network(&network_id).await?;
    Ok(HttpResponse::Ok().json(stations))
}

#[utoipa::path(
    put,
    path = "/api/v1/stations/{station_id}",
    params(
        ("station_id" = String, Path, description = "Station ID")
    ),
    request_body = UpdateStationRequest,
    responses(
        (status = 200, description = "Station updated", body = StationResponse),
        (status = 404, description = "Station not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "stations"
)]
pub async fn update_station(
    req: HttpRequest,
    station_id: web::Path<String>,
    payload: web::Json<UpdateStationRequest>,
    service: web::Data<StationService>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    let station = service
        .update_station(&station_id, payload.into_inner(), claims.jti)
        .await?;
    Ok(HttpResponse::Ok().json(station))
}

#[utoipa::path(
    delete,
    path = "/api/v1/stations/{station_id}",
    params(
        ("station_id" = String, Path, description = "Station ID")
    ),
    responses(
        (status = 204, description = "Station deleted"),
        (status = 404, description = "Station not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "stations"
)]
pub async fn delete_station(
    _req: HttpRequest,
    station_id: web::Path<String>,
    service: web::Data<StationService>,
) -> AppResult<HttpResponse> {
    service.delete_station(&station_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Connector Handlers
#[utoipa::path(
    post,
    path = "/api/v1/connectors",
    request_body = CreateConnectorRequest,
    responses(
        (status = 201, description = "Connector created", body = ConnectorResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "connectors"
)]
pub async fn create_connector(
    req: HttpRequest,
    payload: web::Json<CreateConnectorRequest>,
    service: web::Data<ConnectorService>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    let connector = service.create_connector(payload.into_inner(), claims.jti).await?;
    Ok(HttpResponse::Created().json(connector))
}

#[utoipa::path(
    get,
    path = "/api/v1/connectors/{connector_id}",
    params(
        ("connector_id" = String, Path, description = "Connector ID")
    ),
    responses(
        (status = 200, description = "Connector found", body = ConnectorResponse),
        (status = 404, description = "Connector not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "connectors"
)]
pub async fn get_connector(
    _req: HttpRequest,
    connector_id: web::Path<String>,
    service: web::Data<ConnectorService>,
) -> AppResult<HttpResponse> {
    let connector = service.get_connector(&connector_id).await?;
    Ok(HttpResponse::Ok().json(connector))
}

#[utoipa::path(
    get,
    path = "/api/v1/stations/{station_id}/connectors",
    params(
        ("station_id" = String, Path, description = "Station ID")
    ),
    responses(
        (status = 200, description = "List of connectors", body = Vec<ConnectorResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "connectors"
)]
pub async fn list_connectors_by_station(
    _req: HttpRequest,
    station_id: web::Path<String>,
    service: web::Data<ConnectorService>,
) -> AppResult<HttpResponse> {
    let connectors = service.list_connectors_by_station(&station_id).await?;
    Ok(HttpResponse::Ok().json(connectors))
}

#[utoipa::path(
    put,
    path = "/api/v1/connectors/{connector_id}",
    params(
        ("connector_id" = String, Path, description = "Connector ID")
    ),
    request_body = UpdateConnectorRequest,
    responses(
        (status = 200, description = "Connector updated", body = ConnectorResponse),
        (status = 404, description = "Connector not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "connectors"
)]
pub async fn update_connector(
    req: HttpRequest,
    connector_id: web::Path<String>,
    payload: web::Json<UpdateConnectorRequest>,
    service: web::Data<ConnectorService>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    let connector = service
        .update_connector(&connector_id, payload.into_inner(), claims.jti)
        .await?;
    Ok(HttpResponse::Ok().json(connector))
}

#[utoipa::path(
    delete,
    path = "/api/v1/connectors/{connector_id}",
    params(
        ("connector_id" = String, Path, description = "Connector ID")
    ),
    responses(
        (status = 204, description = "Connector deleted"),
        (status = 404, description = "Connector not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "connectors"
)]
pub async fn delete_connector(
    _req: HttpRequest,
    connector_id: web::Path<String>,
    service: web::Data<ConnectorService>,
) -> AppResult<HttpResponse> {
    service.delete_connector(&connector_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Health check
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    ),
    tag = "health"
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "admin-service"
    }))
}