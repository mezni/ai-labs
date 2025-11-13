use actix_web::{get, web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions, Row};
use std::env;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

// Request and Response models with OpenAPI documentation
#[derive(Debug, Deserialize, ToSchema, OpenApi)]
#[serde(rename_all = "camelCase")]
struct NearbyStationsRequest {
    /// Longitude coordinate (x-axis)
    #[schema(example = 10.1815)]
    longitude: f64,
    
    /// Latitude coordinate (y-axis)  
    #[schema(example = 36.8065)]
    latitude: f64,
    
    /// Search radius in kilometers
    #[schema(example = 5.0, minimum = 0.1, maximum = 100.0)]
    radius_km: Option<f64>,
    
    /// Maximum number of results to return
    #[schema(example = 10, minimum = 1, maximum = 100)]
    max_results: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct NearbyStationsDetailRequest {
    /// Longitude coordinate (x-axis)
    #[schema(example = 10.1815)]
    longitude: f64,
    
    /// Latitude coordinate (y-axis)
    #[schema(example = 36.8065)]
    latitude: f64,
    
    /// Search radius in kilometers
    #[schema(example = 5.0, minimum = 0.1, maximum = 100.0)]
    radius_km: Option<f64>,
    
    /// Minimum power in kW for filtering
    #[schema(example = 50.0, minimum = 0.0, maximum = 1000.0)]
    min_power_kw: Option<f64>,
    
    /// Filter by connector types
    #[schema(example = json!(["CCS Combo 1", "Type 2"]))]
    connector_types: Option<Vec<String>>,
    
    /// Filter by power tiers
    #[schema(example = json!(["ultra_fast", "fast"]))]
    power_tiers: Option<Vec<String>>,
    
    /// Maximum number of results to return
    #[schema(example = 10, minimum = 1, maximum = 100)]
    max_results: Option<i32>,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Station {
    /// Unique station identifier
    station_id: i32,
    
    /// Station name
    name: String,
    
    /// Physical address
    address: String,
    
    /// City where station is located
    city: String,
    
    /// Distance from search location in kilometers
    distance_km: f64,
    
    /// Maximum power available at this station in kW
    max_power_kw: f64,
    
    /// Number of currently available connectors
    available_connectors: i32,
    
    /// Total number of connectors at this station
    total_connectors: i32,
    
    /// Types of connectors available
    connector_types: Vec<String>,
    
    /// Power tier classification
    #[schema(example = "fast")]
    power_tier: String,
    
    /// Whether the station is operational
    is_operational: bool,
    
    /// Station latitude
    latitude: f64,
    
    /// Station longitude  
    longitude: f64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct ApiResponse<T> {
    /// Indicates if the request was successful
    success: bool,
    
    /// The response data
    data: T,
    
    /// Number of items returned
    count: usize,
}

#[derive(Debug, Serialize, ToSchema)]
struct ErrorResponse {
    /// Indicates if the request was successful
    success: bool,
    
    /// Error message description
    error: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct HealthResponse {
    /// Service status
    status: String,
    
    /// Service name
    service: String,
    
    /// API version
    version: String,
    
    /// Timestamp of the check
    timestamp: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct ApiInfo {
    /// API name
    name: String,
    
    /// API version
    version: String,
    
    /// API description
    description: String,
    
    /// Available endpoints
    endpoints: std::collections::HashMap<String, String>,
}

// Database connection pool
struct AppState {
    db_pool: PgPool,
}

// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        find_nearby_stations,
        find_nearby_stations_detail,
        health_check,
        index
    ),
    components(
        schemas(
            NearbyStationsRequest,
            NearbyStationsDetailRequest,
            Station,
            ApiResponse<Station>,
            ErrorResponse,
            HealthResponse,
            ApiInfo
        )
    ),
    tags(
        (name = "Charging Stations", description = "EV Charging Stations API")
    ),
    info(
        title = "Charging Stations API",
        description = "REST API for finding electric vehicle charging stations",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        version = "1.0.0"
    )
)]
struct ApiDoc;

// Helper function to map database row to Station struct
fn row_to_station(row: &sqlx::postgres::PgRow) -> Result<Station, sqlx::Error> {
    let connector_types: Option<Vec<String>> = row.try_get("connector_types")?;
    
    Ok(Station {
        station_id: row.try_get("station_id")?,
        name: row.try_get("name")?,
        address: row.try_get("address")?,
        city: row.try_get("city")?,
        distance_km: row.try_get("distance_km")?,
        max_power_kw: row.try_get("max_power_kw")?,
        available_connectors: row.try_get("available_connectors")?,
        total_connectors: row.try_get("total_connectors")?,
        connector_types: connector_types.unwrap_or_default(),
        power_tier: row.try_get("power_tier")?,
        is_operational: row.try_get("is_operational")?,
        latitude: row.try_get("latitude")?,
        longitude: row.try_get("longitude")?,
    })
}

// Handler for find_nearby_stations
#[utoipa::path(
    context_path = "/api/stations",
    params(
        ("longitude" = f64, Query, description = "Longitude coordinate"),
        ("latitude" = f64, Query, description = "Latitude coordinate"),
        ("radius_km" = Option<f64>, Query, description = "Search radius in kilometers"),
        ("max_results" = Option<i32>, Query, description = "Maximum number of results")
    ),
    responses(
        (status = 200, description = "Successfully found nearby stations", body = ApiResponse<Station>),
        (status = 400, description = "Invalid parameters", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Charging Stations"
)]
#[get("/nearby")]
async fn find_nearby_stations(
    data: web::Data<AppState>,
    query: web::Query<NearbyStationsRequest>,
) -> Result<HttpResponse> {
    let radius_km = query.radius_km.unwrap_or(10.0);
    let max_results = query.max_results.unwrap_or(50);

    // Validate coordinates
    if query.longitude < -180.0 || query.longitude > 180.0 {
        let error_response = ErrorResponse {
            success: false,
            error: "Longitude must be between -180 and 180".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }
    
    if query.latitude < -90.0 || query.latitude > 90.0 {
        let error_response = ErrorResponse {
            success: false,
            error: "Latitude must be between -90 and 90".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // Use dynamic query instead of macro
    let rows = sqlx::query(
        "SELECT * FROM find_nearby_stations($1, $2, $3, $4)"
    )
    .bind(query.longitude)
    .bind(query.latitude)
    .bind(radius_km)
    .bind(max_results)
    .fetch_all(&data.db_pool)
    .await;

    match rows {
        Ok(rows) => {
            let mut stations = Vec::new();
            for row in rows {
                match row_to_station(&row) {
                    Ok(station) => stations.push(station),
                    Err(e) => {
                        log::error!("Error mapping row to station: {}", e);
                        continue;
                    }
                }
            }
            
            let response = ApiResponse {
                success: true,
                data: stations.clone(),
                count: stations.len(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            let error_response = ErrorResponse {
                success: false,
                error: "Internal server error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

// Handler for find_nearby_stations_detail
#[utoipa::path(
    context_path = "/api/stations",
    params(
        ("longitude" = f64, Query, description = "Longitude coordinate"),
        ("latitude" = f64, Query, description = "Latitude coordinate"),
        ("radius_km" = Option<f64>, Query, description = "Search radius in kilometers"),
        ("min_power_kw" = Option<f64>, Query, description = "Minimum power in kW"),
        ("connector_types" = Option<Vec<String>>, Query, description = "Filter by connector types"),
        ("power_tiers" = Option<Vec<String>>, Query, description = "Filter by power tiers"),
        ("max_results" = Option<i32>, Query, description = "Maximum number of results")
    ),
    responses(
        (status = 200, description = "Successfully found nearby stations with details", body = ApiResponse<Station>),
        (status = 400, description = "Invalid parameters", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Charging Stations"
)]
#[get("/nearby/detail")]
async fn find_nearby_stations_detail(
    data: web::Data<AppState>,
    query: web::Query<NearbyStationsDetailRequest>,
) -> Result<HttpResponse> {
    let radius_km = query.radius_km.unwrap_or(10.0);
    let max_results = query.max_results.unwrap_or(50);
    let min_power_kw = query.min_power_kw.unwrap_or(0.0);

    // Validate coordinates
    if query.longitude < -180.0 || query.longitude > 180.0 {
        let error_response = ErrorResponse {
            success: false,
            error: "Longitude must be between -180 and 180".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }
    
    if query.latitude < -90.0 || query.latitude > 90.0 {
        let error_response = ErrorResponse {
            success: false,
            error: "Latitude must be between -90 and 90".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // Use dynamic query instead of macro
    let rows = sqlx::query(
        "SELECT * FROM find_nearby_stations_detail($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(query.longitude)
    .bind(query.latitude)
    .bind(radius_km)
    .bind(min_power_kw)
    .bind(query.connector_types.as_ref())
    .bind(query.power_tiers.as_ref())
    .bind(max_results)
    .fetch_all(&data.db_pool)
    .await;

    match rows {
        Ok(rows) => {
            let mut stations = Vec::new();
            for row in rows {
                match row_to_station(&row) {
                    Ok(station) => stations.push(station),
                    Err(e) => {
                        log::error!("Error mapping row to station: {}", e);
                        continue;
                    }
                }
            }
            
            let response = ApiResponse {
                success: true,
                data: stations.clone(),
                count: stations.len(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            let error_response = ErrorResponse {
                success: false,
                error: "Internal server error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

// Health check endpoint
#[utoipa::path(
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "System"
)]
#[get("/health")]
async fn health_check() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "ok".to_string(),
        service: "charging-stations-api".to_string(),
        version: "1.0.0".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    Ok(HttpResponse::Ok().json(response))
}

// Root endpoint with API information
#[utoipa::path(
    responses(
        (status = 200, description = "API information", body = ApiInfo)
    ),
    tag = "System"
)]
#[get("/")]
async fn index() -> Result<HttpResponse> {
    let mut endpoints = std::collections::HashMap::new();
    endpoints.insert(
        "find_nearby_stations".to_string(),
        "/api/stations/nearby?longitude=10.0&latitude=36.0&radius_km=5&max_results=10".to_string(),
    );
    endpoints.insert(
        "find_nearby_stations_detail".to_string(),
        "/api/stations/nearby/detail?longitude=10.0&latitude=36.0&radius_km=5&min_power_kw=50&max_results=10".to_string(),
    );
    endpoints.insert("health_check".to_string(), "/health".to_string());
    endpoints.insert("swagger_ui".to_string(), "/swagger".to_string());

    let response = ApiInfo {
        name: "Charging Stations API".to_string(),
        version: "1.0.0".to_string(),
        description: "REST API for finding electric vehicle charging stations".to_string(),
        endpoints,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Create database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // Test database connection
    match sqlx::query("SELECT 1").execute(&db_pool).await {
        Ok(_) => println!("✅ Database connection successful"),
        Err(e) => {
            eprintln!("❌ Database connection failed: {}", e);
            std::process::exit(1);
        }
    }

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    println!("🚀 Starting server on http://{}", bind_address);
    println!("📚 Swagger UI available at http://{}/swagger", bind_address);
    println!("🏥 Health check available at http://{}/health", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
            }))
            .service(
                SwaggerUi::new("/swagger/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(index)
            .service(health_check)
            .service(
                web::scope("/api/stations")
                    .service(find_nearby_stations)
                    .service(find_nearby_stations_detail)
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}