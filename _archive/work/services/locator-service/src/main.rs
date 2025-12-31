use actix_web::{
    web, App, HttpServer, HttpResponse, Result, middleware,
    get, HttpRequest
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgRow, Row};
use std::env;
use utoipa::{OpenApi, ToSchema, IntoParams};
use utoipa_swagger_ui::SwaggerUi;
use dotenvy::dotenv;

// ==================== OPENAPI DOCUMENTATION ====================

#[derive(OpenApi)]
#[openapi(
    paths(
        get_nearby_stations,
        get_nearby_stations_details,
        health_check
    ),
    components(
        schemas(
            StationDetail,
            StationSimple,
            ApiResponseStationDetail,
            ApiResponseStationSimple,
            HealthCheck
        )
    ),
    tags(
        (name = "Charging Stations", description = "EV Charging Stations API")
    ),
    info(
        title = "EV Charging Stations API",
        version = "1.0.0",
        description = "API for finding nearby electric vehicle charging stations in Tunisia",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(utoipa::openapi::security::HttpAuthScheme::Bearer)
                )
            )
        }
    }
}

// ==================== MODELS ====================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct StationDetail {
    #[schema(example = 123)]
    pub station_id: i32,
    #[schema(example = 456)]
    pub network_id: i32,
    #[schema(example = "Tunisie Electricity")]
    pub network_name: String,
    #[schema(example = "company")]
    pub network_type: String,
    #[schema(example = "contact@tunisie-electricity.tn")]
    pub contact_email: Option<String>,
    #[schema(example = "+216-70-010-203")]
    pub phone_number: Option<String>,
    #[schema(example = "Rue de la République, Tunis 1000")]
    pub network_address: Option<String>,
    #[schema(example = "Station Chargement Centre Ville")]
    pub station_name: String,
    #[schema(example = "Avenue Habib Bourguiba")]
    pub station_address: String,
    #[schema(example = "Tunis")]
    pub city: Option<String>,
    #[schema(example = "Tunis")]
    pub state: Option<String>,
    #[schema(example = "Tunisia")]
    pub country: Option<String>,
    #[schema(example = "1000")]
    pub postal_code: Option<String>,
    #[schema(example = 10.1815)]
    pub longitude: f64,
    #[schema(example = 36.8065)]
    pub latitude: f64,
    pub tags: Option<Vec<String>>,
    #[schema(example = "verified")]
    pub station_status: String,
    #[schema(example = true)]
    pub is_operational: bool,
    #[schema(example = true)]
    pub has_available_connectors: bool,
    #[schema(example = 4)]
    pub total_connectors: i32,
    #[schema(example = 2)]
    pub total_available_connectors: i32,
    #[schema(example = 350.0)]
    pub total_power_capacity_kw: f64,
    #[schema(example = 150.0)]
    pub available_power_capacity_kw: f64,
    #[schema(example = json!(["CCS2", "Type 2"]))]
    pub available_connector_names: Option<Vec<String>>,
    pub connectors: Option<serde_json::Value>,
    pub network: Option<serde_json::Value>,
    #[schema(example = "24/7")]
    pub opening_hours: Option<String>,
    #[schema(example = 10)]
    pub capacity: Option<i32>,
    #[schema(example = "0.450 TND/kWh")]
    pub fee: Option<String>,
    #[schema(example = "Gratuit")]
    pub parking_fee: Option<String>,
    #[schema(example = "public")]
    pub access: Option<String>,
}

impl StationDetail {
    pub fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            station_id: row.try_get("station_id")?,
            network_id: row.try_get("network_id")?,
            network_name: row.try_get("network_name")?,
            network_type: row.try_get("network_type")?,
            contact_email: row.try_get("contact_email")?,
            phone_number: row.try_get("phone_number")?,
            network_address: row.try_get("network_address")?,
            station_name: row.try_get("station_name")?,
            station_address: row.try_get("station_address")?,
            city: row.try_get("city")?,
            state: row.try_get("state")?,
            country: row.try_get("country")?,
            postal_code: row.try_get("postal_code")?,
            longitude: row.try_get("longitude")?,
            latitude: row.try_get("latitude")?,
            tags: row.try_get("tags")?,
            station_status: row.try_get("station_status")?,
            is_operational: row.try_get("is_operational")?,
            has_available_connectors: row.try_get("has_available_connectors")?,
            total_connectors: row.try_get("total_connectors")?,
            total_available_connectors: row.try_get("total_available_connectors")?,
            total_power_capacity_kw: row.try_get("total_power_capacity_kw")?,
            available_power_capacity_kw: row.try_get("available_power_capacity_kw")?,
            available_connector_names: row.try_get("available_connector_names")?,
            connectors: row.try_get("connectors")?,
            network: row.try_get("network")?,
            opening_hours: row.try_get("opening_hours")?,
            capacity: row.try_get("capacity")?,
            fee: row.try_get("fee")?,
            parking_fee: row.try_get("parking_fee")?,
            access: row.try_get("access")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct StationSimple {
    #[schema(example = 123)]
    pub station_id: i32,
    #[schema(example = 456)]
    pub network_id: i32,
    #[schema(example = "Tunisie Electricity")]
    pub network_name: String,
    #[schema(example = "+216-70-010-203")]
    pub phone_number: Option<String>,
    #[schema(example = "Station Chargement Centre Ville")]
    pub station_name: String,
    #[schema(example = "Avenue Habib Bourguiba")]
    pub station_address: String,
    #[schema(example = "Tunis")]
    pub city: Option<String>,
    #[schema(example = "Tunis")]
    pub state: Option<String>,
    #[schema(example = "Tunisia")]
    pub country: Option<String>,
    #[schema(example = 10.1815)]
    pub longitude: f64,
    #[schema(example = 36.8065)]
    pub latitude: f64,
    pub tags: Option<Vec<String>>,
    #[schema(example = "verified")]
    pub station_status: String,
    #[schema(example = true)]
    pub is_operational: bool,
    #[schema(example = true)]
    pub has_available_connectors: bool,
    #[schema(example = 4)]
    pub total_connectors: i32,
    #[schema(example = 2)]
    pub total_available_connectors: i32,
    #[schema(example = 350.0)]
    pub total_power_capacity_kw: f64,
    #[schema(example = 150.0)]
    pub available_power_capacity_kw: f64,
    #[schema(example = json!(["CCS2", "Type 2"]))]
    pub available_connector_names: Option<Vec<String>>,
    pub network: Option<serde_json::Value>,
    #[schema(example = "24/7")]
    pub opening_hours: Option<String>,
    #[schema(example = 10)]
    pub capacity: Option<i32>,
    #[schema(example = "0.450 TND/kWh")]
    pub fee: Option<String>,
    #[schema(example = "Gratuit")]
    pub parking_fee: Option<String>,
    #[schema(example = "public")]
    pub access: Option<String>,
}

impl StationSimple {
    pub fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            station_id: row.try_get("station_id")?,
            network_id: row.try_get("network_id")?,
            network_name: row.try_get("network_name")?,
            phone_number: row.try_get("phone_number")?,
            station_name: row.try_get("station_name")?,
            station_address: row.try_get("station_address")?,
            city: row.try_get("city")?,
            state: row.try_get("state")?,
            country: row.try_get("country")?,
            longitude: row.try_get("longitude")?,
            latitude: row.try_get("latitude")?,
            tags: row.try_get("tags")?,
            station_status: row.try_get("station_status")?,
            is_operational: row.try_get("is_operational")?,
            has_available_connectors: row.try_get("has_available_connectors")?,
            total_connectors: row.try_get("total_connectors")?,
            total_available_connectors: row.try_get("total_available_connectors")?,
            total_power_capacity_kw: row.try_get("total_power_capacity_kw")?,
            available_power_capacity_kw: row.try_get("available_power_capacity_kw")?,
            available_connector_names: row.try_get("available_connector_names")?,
            network: row.try_get("network")?,
            opening_hours: row.try_get("opening_hours")?,
            capacity: row.try_get("capacity")?,
            fee: row.try_get("fee")?,
            parking_fee: row.try_get("parking_fee")?,
            access: row.try_get("access")?,
        })
    }
}

// ==================== API RESPONSE ====================

#[derive(Debug, Serialize, ToSchema)]
struct ApiResponseStationDetail {
    #[schema(example = true)]
    success: bool,
    data: Option<Vec<StationDetail>>,
    #[schema(example = "Error message if any")]
    error: Option<String>,
}

impl ApiResponseStationDetail {
    fn success(data: Vec<StationDetail>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
struct ApiResponseStationSimple {
    #[schema(example = true)]
    success: bool,
    data: Option<Vec<StationSimple>>,
    #[schema(example = "Error message if any")]
    error: Option<String>,
}

impl ApiResponseStationSimple {
    fn success(data: Vec<StationSimple>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// ==================== QUERY PARAMS ====================

#[derive(Debug, Deserialize, IntoParams)]
struct BasicLocationQuery {
    #[param(example = 10.1815)]
    longitude: f64,
    #[param(example = 36.8065)]
    latitude: f64,
    #[param(example = 10.0)]
    #[serde(default = "default_radius_basic")]
    radius_km: f64,
    #[param(example = 50)]
    #[serde(default = "default_limit_basic")]
    limit: i32,
    #[param(example = 0)]
    #[serde(default)]
    offset: i32,
}

#[derive(Debug, Deserialize, IntoParams)]
struct DetailedLocationQuery {
    #[param(example = 10.1815)]
    longitude: f64,
    #[param(example = 36.8065)]
    latitude: f64,
    #[param(example = 10.0)]
    #[serde(default = "default_radius_detailed")]
    radius_km: f64,
    #[param(example = 50.0)]
    min_power_kw: Option<f64>,
    #[param(example = "CCS2,Type 2")]
    connector_types: Option<String>,
    #[param(example = "fast,ultra_fast")]
    power_tiers: Option<String>,
    #[param(example = 50)]
    #[serde(default = "default_limit_detailed")]
    limit: i32,
    #[param(example = 0)]
    #[serde(default)]
    offset: i32,
}

fn default_radius_basic() -> f64 { 10.0 }
fn default_radius_detailed() -> f64 { 10.0 }
fn default_limit_basic() -> i32 { 50 }
fn default_limit_detailed() -> i32 { 50 }

// Helper function to convert comma-separated string to array
fn parse_string_array(input: Option<&String>) -> Option<Vec<String>> {
    input.map(|s| s.split(',').map(|item| item.trim().to_string()).collect())
}

// ==================== AUTHENTICATION ====================

fn extract_bearer_token(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header[7..].to_string())
            } else {
                None
            }
        })
}

fn is_authenticated(req: &HttpRequest) -> bool {
    extract_bearer_token(req).is_some()
}

// ==================== API HANDLERS ====================

/// Get nearby charging stations (Basic)
/// 
/// Returns basic information about charging stations near the specified location in Tunisia.
/// This endpoint is publicly accessible and does not require authentication.
/// 
/// **Available filters:**
/// - `longitude` (required): Longitude coordinate
/// - `latitude` (required): Latitude coordinate  
/// - `radius_km` (optional, default: 10.0): Search radius in kilometers
/// - `limit` (optional, default: 50): Maximum number of results
/// - `offset` (optional, default: 0): Pagination offset
#[utoipa::path(
    context_path = "/api/stations",
    params(BasicLocationQuery),
    responses(
        (status = 200, description = "List of nearby charging stations", body = ApiResponseStationSimple),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/nearby")]
async fn get_nearby_stations(
    query: web::Query<BasicLocationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let stations = sqlx::query(
        "SELECT * FROM find_nearby_stations($1, $2, $3, $4, $5)"
    )
    .bind(query.longitude)
    .bind(query.latitude)
    .bind(query.radius_km)
    .bind(query.limit)
    .bind(query.offset)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?
    .iter()
    .map(StationSimple::from_row)
    .collect::<Result<Vec<StationSimple>, sqlx::Error>>()
    .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Row parsing error: {}", e)))?;

    Ok(HttpResponse::Ok().json(ApiResponseStationSimple::success(stations)))
}

/// Get nearby charging stations with detailed information
/// 
/// Returns detailed information about charging stations near the specified location in Tunisia.
/// This endpoint requires authentication and provides sensitive details like contact information.
/// 
/// **Available filters:**
/// - `longitude` (required): Longitude coordinate
/// - `latitude` (required): Latitude coordinate
/// - `radius_km` (optional, default: 10.0): Search radius in kilometers
/// - `min_power_kw` (optional): Minimum power capacity in kW
/// - `connector_types` (optional): Comma-separated connector types (e.g., "CCS2,Type 2")
/// - `power_tiers` (optional): Comma-separated power tiers (e.g., "fast,ultra_fast")
/// - `limit` (optional, default: 50): Maximum number of results
/// - `offset` (optional, default: 0): Pagination offset
#[utoipa::path(
    context_path = "/api/stations",
    params(DetailedLocationQuery),
    responses(
        (status = 200, description = "List of nearby charging stations with detailed information", body = ApiResponseStationDetail),
        (status = 401, description = "Unauthorized - Authentication required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/nearby-details")]
async fn get_nearby_stations_details(
    req: HttpRequest,
    query: web::Query<DetailedLocationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    // Check authentication for detailed endpoint
    if !is_authenticated(&req) {
        return Ok(HttpResponse::Unauthorized().json(ApiResponseStationDetail::error(
            "Authentication required. Please provide a valid Bearer token.".to_string()
        )));
    }

    // Convert comma-separated strings to arrays for PostgreSQL
    let connector_types_array = parse_string_array(query.connector_types.as_ref());
    let power_tiers_array = parse_string_array(query.power_tiers.as_ref());

    let stations = sqlx::query(
        "SELECT * FROM find_nearby_stations_detail($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(query.longitude)
    .bind(query.latitude)
    .bind(query.radius_km)
    .bind(query.min_power_kw)
    .bind(&connector_types_array)
    .bind(&power_tiers_array)
    .bind(query.limit)
    .bind(query.offset)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?
    .iter()
    .map(StationDetail::from_row)
    .collect::<Result<Vec<StationDetail>, sqlx::Error>>()
    .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Row parsing error: {}", e)))?;

    Ok(HttpResponse::Ok().json(ApiResponseStationDetail::success(stations)))
}

// ==================== HEALTH CHECK ====================

#[derive(Debug, Serialize, ToSchema)]
struct HealthCheck {
    #[schema(example = "ok")]
    status: String,
    #[schema(example = "connected")]
    database: String,
}

/// Health check endpoint
/// 
/// Returns the API status and database connection status.
#[utoipa::path(
    responses(
        (status = 200, description = "API health status", body = HealthCheck)
    )
)]
#[get("/health")]
async fn health_check(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let db_status = sqlx::query("SELECT 1")
        .fetch_optional(pool.get_ref())
        .await
        .map(|_| "connected")
        .unwrap_or_else(|_| "disconnected");

    let health = HealthCheck {
        status: "ok".to_string(),
        database: db_status.to_string(),
    };

    Ok(HttpResponse::Ok().json(health))
}

// ==================== MAIN ====================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "5800".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid number");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create database pool");

    println!("🚀 Starting Tunisia Charging Stations API on http://{}:{}", server_host, server_port);
    println!("📍 Basic nearby stations: GET /api/stations/nearby (No auth required)");
    println!("🔐 Detailed nearby stations: GET /api/stations/nearby-details (Auth required)");
    println!("❤️  Health check: GET /health");
    println!("📚 Swagger UI: http://{}:{}/swagger-ui/", server_host, server_port);
    println!("📖 OpenAPI spec: http://{}:{}/api-docs/openapi.json", server_host, server_port);
    println!("");
    println!("🔑 To test authenticated endpoints in Swagger UI:");
    println!("   1. Click the 'Authorize' button at the top");
    println!("   2. Enter your Bearer token (e.g., 'test-token-123')");
    println!("   3. Click 'Authorize' and close the dialog");
    println!("");
    println!("🇹🇳 Example queries for Tunis:");
    println!("Basic: GET /api/stations/nearby?longitude=10.1815&latitude=36.8065&radius_km=5");
    println!("Detailed: GET /api/stations/nearby-details?longitude=10.1815&latitude=36.8065&radius_km=5&min_power_kw=50&connector_types=CCS2,Type2");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/stations")
                    .service(get_nearby_stations)
                    .service(get_nearby_stations_details)
            )
            .service(health_check)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}