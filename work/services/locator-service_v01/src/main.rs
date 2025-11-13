use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Result, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use rust_decimal::prelude::*;

// Database connection pool type
type DbPool = Pool<Postgres>;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NearbyStation {
    pub station_id: i32,
    pub name: String,
    pub address: String,
    pub city: Option<String>,
    pub distance_km: f64,
    pub max_power_kw: Option<f64>,
    pub available_connectors: i32,
    pub total_connectors: i32,
    pub connector_types: Option<Vec<String>>,
    pub power_tier: Option<String>,
    pub is_operational: Option<bool>, // Made optional
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StationDetails {
    pub station_id: i32,
    pub name: String,
    pub address: String,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub max_power_kw: Option<f64>,
    pub available_connectors: i32,
    pub total_connectors: i32,
    pub connector_types: Option<Vec<String>>,
    pub power_tier: Option<String>,
    pub connectors: Option<serde_json::Value>,
    pub tags: Option<serde_json::Value>,
    pub network_name: Option<String>,
    pub is_operational: Option<bool>, // Made optional
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConnectorType {
    pub connector_type_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub current_type: String,
    pub typical_power_kw: Option<f64>,
    pub standard: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NearbyQuery {
    pub longitude: f64,
    pub latitude: f64,
    pub radius_km: Option<f64>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AdvancedNearbyQuery {
    pub longitude: f64,
    pub latitude: f64,
    pub radius_km: Option<f64>,
    pub min_power_kw: Option<f64>,
    pub connector_types: Option<Vec<String>>,
    pub power_tiers: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
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

// Helper function to convert BigDecimal to f64
fn bigdecimal_to_f64(decimal: Option<sqlx::types::BigDecimal>) -> Option<f64> {
    decimal.and_then(|d| d.to_f64())
}
// ... (keep all the imports and struct definitions the same)

// API Handlers with better error logging
#[get("/api/stations/nearby")]
async fn get_nearby_stations(
    pool: web::Data<DbPool>,
    query: web::Query<NearbyQuery>,
) -> Result<HttpResponse> {
    log::info!("Searching nearby stations: longitude={}, latitude={}, radius={}km", 
        query.longitude, query.latitude, query.radius_km.unwrap_or(10.0));
    
    let stations = sqlx::query_as::<_, NearbyStation>(
        "SELECT * FROM find_nearby_stations($1, $2, $3, $4, $5)"
    )
    .bind(query.longitude)
    .bind(query.latitude)
    .bind(query.radius_km.unwrap_or(10.0))
    .bind(query.limit.unwrap_or(50))
    .bind(query.offset.unwrap_or(0))
    .fetch_all(pool.get_ref())
    .await;

    match stations {
        Ok(stations) => {
            log::info!("Found {} stations", stations.len());
            Ok(HttpResponse::Ok().json(ApiResponse::success(stations)))
        },
        Err(e) => {
            log::error!("Database error in find_nearby_stations: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<NearbyStation>>::error(
                format!("Failed to fetch nearby stations: {}", e)
            )))
        }
    }
}

#[get("/api/stations/nearby/detailed")]
async fn get_nearby_stations_detailed(
    pool: web::Data<DbPool>,
    query: web::Query<AdvancedNearbyQuery>,
) -> Result<HttpResponse> {
    log::info!("Detailed station search: longitude={}, latitude={}, filters: min_power={:?}, connectors={:?}", 
        query.longitude, query.latitude, query.min_power_kw, query.connector_types);
    
    let stations = sqlx::query_as::<_, NearbyStation>(
        "SELECT * FROM find_nearby_stations_detail($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(query.longitude)
    .bind(query.latitude)
    .bind(query.radius_km.unwrap_or(10.0))
    .bind(query.min_power_kw)
    .bind(&query.connector_types)
    .bind(&query.power_tiers)
    .bind(query.limit.unwrap_or(50))
    .bind(query.offset.unwrap_or(0))
    .fetch_all(pool.get_ref())
    .await;

    match stations {
        Ok(stations) => {
            log::info!("Found {} stations with detailed filters", stations.len());
            Ok(HttpResponse::Ok().json(ApiResponse::success(stations)))
        },
        Err(e) => {
            log::error!("Database error in find_nearby_stations_detail: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<NearbyStation>>::error(
                format!("Failed to fetch nearby stations: {}", e)
            )))
        }
    }
}

// ... (keep other handlers the same with similar error logging)
#[get("/api/stations/{station_id}")]
async fn get_station_details(
    pool: web::Data<DbPool>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let station_id = path.into_inner();
    
    let station = sqlx::query_as::<_, StationDetails>(
        "SELECT * FROM get_station_details($1)"
    )
    .bind(station_id)
    .fetch_optional(pool.get_ref())
    .await;

    match station {
        Ok(Some(station)) => Ok(HttpResponse::Ok().json(ApiResponse::success(station))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<StationDetails>::error(
            "Station not found".to_string()
        ))),
        Err(e) => {
            log::error!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<StationDetails>::error(
                "Failed to fetch station details".to_string()
            )))
        }
    }
}

#[get("/api/connector-types")]
async fn get_connector_types(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let connector_types = sqlx::query_as::<_, ConnectorType>(
        "SELECT * FROM get_connector_types()"
    )
    .fetch_all(pool.get_ref())
    .await;

    match connector_types {
        Ok(types) => Ok(HttpResponse::Ok().json(ApiResponse::success(types))),
        Err(e) => {
            log::error!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<ConnectorType>>::error(
                "Failed to fetch connector types".to_string()
            )))
        }
    }
}

#[get("/api/stats")]
async fn get_system_stats(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    #[derive(Debug, Serialize)]
    struct SystemStats {
        total_stations: i64,
        total_connectors: i64,
        available_connectors: i64,
        avg_power_kw: Option<f64>,
        max_power_kw: Option<f64>,
    }

    let stats = sqlx::query!(
        r#"
        SELECT 
            COUNT(DISTINCT s.station_id) as total_stations,
            COUNT(c.connector_id) as total_connectors,
            COUNT(CASE WHEN c.status = 'available' THEN 1 END) as available_connectors,
            AVG(c.power_level_kw) as avg_power_kw,
            MAX(c.power_level_kw) as max_power_kw
        FROM stations s
        LEFT JOIN connectors c ON s.station_id = c.station_id
        WHERE s.status = 'verified'
        "#
    )
    .fetch_one(pool.get_ref())
    .await;

    match stats {
        Ok(record) => {
            let stats = SystemStats {
                total_stations: record.total_stations.unwrap_or(0),
                total_connectors: record.total_connectors.unwrap_or(0),
                available_connectors: record.available_connectors.unwrap_or(0),
                avg_power_kw: bigdecimal_to_f64(record.avg_power_kw),
                max_power_kw: bigdecimal_to_f64(record.max_power_kw),
            };
            Ok(HttpResponse::Ok().json(ApiResponse::success(stats)))
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<SystemStats>::error(
                "Failed to fetch system stats".to_string()
            )))
        }
    }
}

#[get("/api/health")]
async fn health_check(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    // Simple database health check
    let health_check = sqlx::query("SELECT 1")
        .execute(pool.get_ref())
        .await;

    match health_check {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("API and database are healthy"))),
        Err(e) => {
            log::error!("Database health check failed: {}", e);
            Ok(HttpResponse::ServiceUnavailable().json(ApiResponse::<&str>::error(
                "Database connection failed".to_string()
            )))
        }
    }
}

// Database connection setup
async fn create_db_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ev_db".to_string());

    PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create database pool
    let pool = create_db_pool().await.map_err(|e| {
        eprintln!("Failed to create database pool: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed")
    })?;

    println!("🚀 Charging Stations API server starting on http://0.0.0.0:8080");
    println!("📊 Database connected successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(get_nearby_stations)
            .service(get_nearby_stations_detailed)
            .service(get_station_details)
            .service(get_connector_types)
            .service(get_system_stats)
            .service(health_check)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}