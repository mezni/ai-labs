use actix_web::{
    web, App, HttpServer, HttpResponse, Result, middleware,
    get, HttpRequest
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgRow, Row};
use std::env;
use utoipa::{OpenApi, ToSchema, IntoParams};
use utoipa_swagger_ui::SwaggerUi;

// ==================== OPENAPI DOCUMENTATION ====================

#[derive(OpenApi)]
#[openapi(
    paths(
        get_nearby_stations,
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
        description = "API for finding nearby electric vehicle charging stations",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;

// ==================== MODELS ====================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct StationDetail {
    #[schema(example = 123)]
    pub station_id: i32,
    #[schema(example = 456)]
    pub network_id: i32,
    #[schema(example = "ElectroCharge Network")]
    pub network_name: String,
    #[schema(example = "company")]
    pub network_type: String,
    #[schema(example = "contact@electrocharge.com")]
    pub contact_email: Option<String>,
    #[schema(example = "+1-555-0123")]
    pub phone_number: Option<String>,
    #[schema(example = "123 Energy St, San Francisco, CA")]
    pub network_address: Option<String>,
    #[schema(example = "Downtown Supercharger")]
    pub station_name: String,
    #[schema(example = "456 Main Street")]
    pub station_address: String,
    #[schema(example = "San Francisco")]
    pub city: Option<String>,
    #[schema(example = "CA")]
    pub state: Option<String>,
    #[schema(example = "USA")]
    pub country: Option<String>,
    #[schema(example = "94105")]
    pub postal_code: Option<String>,
    #[schema(example = -122.4194)]
    pub longitude: f64,
    #[schema(example = 37.7749)]
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
    #[schema(example = json!(["CCS", "Type 2"]))]
    pub available_connector_names: Option<Vec<String>>,
    pub connectors: Option<serde_json::Value>,
    pub network: Option<serde_json::Value>,
    #[schema(example = "24/7")]
    pub opening_hours: Option<String>,
    #[schema(example = 10)]
    pub capacity: Option<i32>,
    #[schema(example = "Free for members")]
    pub fee: Option<String>,
    #[schema(example = "$2/hour")]
    pub parking_fee: Option<String>,
    #[schema(example = "public")]
    pub access: Option<String>,
}

impl StationDetail {
    pub fn from_row(row: &PgRow) -> Result<Self, Box<dyn std::error::Error>> {
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
    #[schema(example = "ElectroCharge Network")]
    pub network_name: String,
    #[schema(example = "+1-555-0123")]
    pub phone_number: Option<String>,
    #[schema(example = "Downtown Supercharger")]
    pub station_name: String,
    #[schema(example = "456 Main Street")]
    pub station_address: String,
    #[schema(example = "San Francisco")]
    pub city: Option<String>,
    #[schema(example = "CA")]
    pub state: Option<String>,
    #[schema(example = "USA")]
    pub country: Option<String>,
    #[schema(example = -122.4194)]
    pub longitude: f64,
    #[schema(example = 37.7749)]
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
    #[schema(example = json!(["CCS", "Type 2"]))]
    pub available_connector_names: Option<Vec<String>>,
    pub network: Option<serde_json::Value>,
    #[schema(example = "24/7")]
    pub opening_hours: Option<String>,
    #[schema(example = 10)]
    pub capacity: Option<i32>,
    #[schema(example = "Free for members")]
    pub fee: Option<String>,
    #[schema(example = "$2/hour")]
    pub parking_fee: Option<String>,
    #[schema(example = "public")]
    pub access: Option<String>,
}

impl StationSimple {
    pub fn from_row(row: &PgRow) -> Result<Self, Box<dyn std::error::Error>> {
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
#[schema(title = "ApiResponseStationDetail")]
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
#[schema(title = "ApiResponseStationSimple")]
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
struct LocationQuery {
    #[param(example = -122.4194)]
    longitude: f64,
    #[param(example = 37.7749)]
    latitude: f64,
    #[param(example = 10.0)]
    #[serde(default = "default_radius")]
    radius_km: f64,
    #[param(example = 50.0)]
    min_power_kw: Option<f64>,
    #[param(example = "CCS,Type 2")]
    connector_types: Option<String>,
    #[param(example = "fast,ultra_fast")]
    power_tiers: Option<String>,
    #[param(example = 50)]
    #[serde(default = "default_limit")]
    limit: i32,
    #[param(example = 0)]
    #[serde(default)]
    offset: i32,
}

fn default_radius() -> f64 { 10.0 }
fn default_limit() -> i32 { 50 }

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

/// Get nearby charging stations
/// 
/// Returns a list of charging stations near the specified location.
/// 
/// - **Authenticated users**: Receive detailed station information including contact details, connectors, etc.
/// - **Unauthenticated users**: Receive basic station information without sensitive details.
#[utoipa::path(
    context_path = "/api/stations",
    params(LocationQuery),
    responses(
        (status = 200, description = "List of nearby charging stations", body = inline(ApiResponseStationDetail)),
        (status = 500, description = "Internal server error")
    ),
    security(
        (),
        ("bearer" = [])
    )
)]
#[get("/nearby")]
async fn get_nearby_stations(
    req: HttpRequest,
    query: web::Query<LocationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let is_auth = is_authenticated(&req);
    
    if is_auth {
        get_nearby_stations_detail(query, pool).await
    } else {
        get_nearby_stations_simple(query, pool).await
    }
}

async fn get_nearby_stations_detail(
    query: web::Query<LocationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let stations = sqlx::query(
        r#"
        SELECT * FROM find_nearby_stations_detail(
            $1, $2, $3, $4, 
            CASE WHEN $5 IS NOT NULL THEN string_to_array($5, ',')::text[] ELSE NULL END,
            CASE WHEN $6 IS NOT NULL THEN string_to_array($6, ',')::text[] ELSE NULL END,
            $7, $8
        )
        "#
    )
    .bind(query.longitude)
    .bind(query.latitude)
    .bind(query.radius_km)
    .bind(query.min_power_kw)
    .bind(query.connector_types.as_deref())
    .bind(query.power_tiers.as_deref())
    .bind(query.limit)
    .bind(query.offset)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?
    .iter()
    .map(StationDetail::from_row)
    .collect::<Result<Vec<StationDetail>, _>>()
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(ApiResponseStationDetail::success(stations)))
}

async fn get_nearby_stations_simple(
    query: web::Query<LocationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let stations = sqlx::query(
        r#"
        SELECT * FROM find_nearby_stations(
            $1, $2, $3, $4, $5
        )
        "#
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
    .collect::<Result<Vec<StationSimple>, _>>()
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(ApiResponseStationSimple::success(stations)))
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
    dotenvy::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create database pool");

    println!("🚀 Starting Charging Stations API on http://localhost:8080");
    println!("📍 Nearby stations endpoint: GET /api/stations/nearby");
    println!("❤️  Health check: GET /health");
    println!("📚 Swagger UI: http://localhost:8080/swagger-ui/");
    println!("📖 OpenAPI spec: http://localhost:8080/api-docs/openapi.json");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/stations")
                    .service(get_nearby_stations)
            )
            .service(health_check)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}