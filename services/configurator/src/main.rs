use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, error, warn};
use tracing_subscriber::EnvFilter;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use thiserror::Error;

// === CONSTANTS ===
const API_PREFIX: &str = "/api/v1";
const DEFAULT_MAX_CONNECTIONS: u32 = 10;

// === ERROR TYPES ===
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Connection pool error: {0}")]
    PoolError(String),
    
    #[error("Network not found")]
    NotFound,
    
    #[error("Failed to create network: {0}")]
    CreateFailed(String),
    
    #[error("Failed to update network: {0}")]
    UpdateFailed(String),
    
    #[error("Failed to delete network: {0}")]
    DeleteFailed(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl actix_web::ResponseError for NetworkError {
    fn error_response(&self) -> HttpResponse {
        match self {
            NetworkError::NotFound => HttpResponse::NotFound().json(self.to_string()),
            NetworkError::CreateFailed(_) => HttpResponse::InternalServerError().json(self.to_string()),
            NetworkError::UpdateFailed(_) => HttpResponse::InternalServerError().json(self.to_string()),
            NetworkError::DeleteFailed(_) => HttpResponse::InternalServerError().json(self.to_string()),
            NetworkError::Validation(_) => HttpResponse::BadRequest().json(self.to_string()),
            NetworkError::ServiceUnavailable(_) => HttpResponse::ServiceUnavailable().json(self.to_string()),
            NetworkError::Database(e) => {
                error!("Database error: {}", e);
                HttpResponse::InternalServerError().json("Database error occurred")
            }
            NetworkError::PoolError(e) => {
                error!("Connection pool error: {}", e);
                HttpResponse::ServiceUnavailable().json("Service temporarily unavailable")
            }
        }
    }
}

// === CONNECTION MANAGER ===
#[derive(Clone)]
pub struct ConnectionManager {
    pool: Arc<PgPool>,
}

impl ConnectionManager {
    pub async fn new(database_url: &str) -> Result<Self, NetworkError> {
        Self::with_config(database_url, DEFAULT_MAX_CONNECTIONS).await
    }

    pub async fn with_config(database_url: &str, max_connections: u32) -> Result<Self, NetworkError> {
        info!("Initializing connection pool with {} max connections", max_connections);

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await
            .map_err(|e| {
                error!("Failed to create connection pool: {}", e);
                NetworkError::PoolError(e.to_string())
            })?;

        // Test the connection
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .map_err(|e| {
                error!("Failed to test database connection: {}", e);
                NetworkError::PoolError(e.to_string())
            })?;

        info!("Connection pool initialized successfully");

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn health_check(&self) -> Result<(), NetworkError> {
        sqlx::query("SELECT 1")
            .execute(self.get_pool())
            .await
            .map_err(|e| {
                error!("Health check failed: {}", e);
                NetworkError::PoolError(e.to_string())
            })?;
        Ok(())
    }

    pub async fn get_connection_stats(&self) -> Result<PoolStats, NetworkError> {
        let size = self.pool.size();
        let num_idle = self.pool.num_idle();
        let num_used = size as u32 - num_idle as u32;

        Ok(PoolStats {
            total_connections: size as u32,
            idle_connections: num_idle as u32,
            used_connections: num_used,
            max_connections: self.pool.options().get_max_connections(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PoolStats {
    pub total_connections: u32,
    pub idle_connections: u32,
    pub used_connections: u32,
    pub max_connections: u32,
}

// === DATA MODELS ===

// DB model returned in responses
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Network {
    pub network_id: i32,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub type_: String,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub created_by: String,
    pub updated_by: Option<String>,
    #[schema(value_type = String)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String)]
    pub updated_at: NaiveDateTime,
}

// DTO for creating a network
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NetworkCreate {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub created_by: String,
}

// DTO for updating a network
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NetworkUpdate {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub contact_email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub updated_by: String,
}

// === SERVICE ===
#[derive(Clone)]
pub struct NetworkService {
    connection_manager: Arc<ConnectionManager>,
}

impl NetworkService {
    pub async fn new(database_url: &str) -> Result<Self, NetworkError> {
        let connection_manager = ConnectionManager::new(database_url).await?;
        
        // Test the connection and verify the table exists
        sqlx::query("SELECT 1 FROM networks LIMIT 1")
            .execute(connection_manager.get_pool())
            .await
            .map_err(|e| {
                error!("Failed to verify networks table: {}", e);
                NetworkError::Database(e)
            })?;
            
        Ok(Self { 
            connection_manager: Arc::new(connection_manager) 
        })
    }

    pub async fn new_with_config(database_url: &str, max_connections: u32) -> Result<Self, NetworkError> {
        let connection_manager = ConnectionManager::with_config(database_url, max_connections).await?;
        
        // Test the connection and verify the table exists
        sqlx::query("SELECT 1 FROM networks LIMIT 1")
            .execute(connection_manager.get_pool())
            .await
            .map_err(|e| {
                error!("Failed to verify networks table: {}", e);
                NetworkError::Database(e)
            })?;
            
        Ok(Self { 
            connection_manager: Arc::new(connection_manager) 
        })
    }

    pub fn get_pool(&self) -> &PgPool {
        self.connection_manager.get_pool()
    }

    pub async fn get_all(&self) -> Result<Vec<Network>, NetworkError> {
        sqlx::query_as::<_, Network>("SELECT * FROM networks")
            .fetch_all(self.get_pool())
            .await
            .map_err(NetworkError::Database)
    }

    pub async fn get_by_id(&self, network_id: i32) -> Result<Option<Network>, NetworkError> {
        sqlx::query_as::<_, Network>("SELECT * FROM networks WHERE network_id = $1")
            .bind(network_id)
            .fetch_optional(self.get_pool())
            .await
            .map_err(NetworkError::Database)
    }

    pub async fn create(&self, network: NetworkCreate) -> Result<Network, NetworkError> {
        // Validate network type
        if network.type_ != "individual" && network.type_ != "company" {
            return Err(NetworkError::Validation(
                "Network type must be either 'individual' or 'company'".to_string()
            ));
        }

        // Validate email if provided
        if let Some(ref email) = network.contact_email {
            if !is_valid_email(email) {
                return Err(NetworkError::Validation(
                    "Invalid email format".to_string()
                ));
            }
        }

        let result = sqlx::query_as::<_, Network>(
            r#"
            INSERT INTO networks (name, type, contact_email, phone_number, address, created_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(&network.name)
        .bind(&network.type_)
        .bind(&network.contact_email)
        .bind(&network.phone_number)
        .bind(&network.address)
        .bind(&network.created_by)
        .fetch_one(self.get_pool())
        .await;

        match result {
            Ok(network) => Ok(network),
            Err(e) => {
                error!("Failed to create network: {}", e);
                Err(NetworkError::CreateFailed(e.to_string()))
            }
        }
    }

    pub async fn update(&self, network_id: i32, network: NetworkUpdate) -> Result<Network, NetworkError> {
        // Validate network type
        if network.type_ != "individual" && network.type_ != "company" {
            return Err(NetworkError::Validation(
                "Network type must be either 'individual' or 'company'".to_string()
            ));
        }

        // Validate email if provided
        if let Some(ref email) = network.contact_email {
            if !is_valid_email(email) {
                return Err(NetworkError::Validation(
                    "Invalid email format".to_string()
                ));
            }
        }

        let result = sqlx::query_as::<_, Network>(
            r#"
            UPDATE networks
            SET name = $1, type = $2, contact_email = $3, phone_number = $4,
                address = $5, updated_by = $6
            WHERE network_id = $7
            RETURNING *
            "#,
        )
        .bind(&network.name)
        .bind(&network.type_)
        .bind(&network.contact_email)
        .bind(&network.phone_number)
        .bind(&network.address)
        .bind(&network.updated_by)
        .bind(network_id)
        .fetch_one(self.get_pool())
        .await;

        match result {
            Ok(network) => Ok(network),
            Err(sqlx::Error::RowNotFound) => Err(NetworkError::NotFound),
            Err(e) => {
                error!("Failed to update network: {}", e);
                Err(NetworkError::UpdateFailed(e.to_string()))
            }
        }
    }

    pub async fn delete(&self, network_id: i32) -> Result<bool, NetworkError> {
        let result = sqlx::query("DELETE FROM networks WHERE network_id = $1")
            .bind(network_id)
            .execute(self.get_pool())
            .await
            .map_err(NetworkError::Database)?;

        if result.rows_affected() > 0 {
            Ok(true)
        } else {
            Err(NetworkError::NotFound)
        }
    }

    // Health check method
    pub async fn health_check(&self) -> Result<(), NetworkError> {
        self.connection_manager.health_check().await
    }

    // Get connection pool statistics
    pub async fn get_pool_stats(&self) -> Result<PoolStats, NetworkError> {
        self.connection_manager.get_connection_stats().await
    }
}

// Simple email validation function
fn is_valid_email(email: &str) -> bool {
    // Basic email validation without regex
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    
    let local_part = parts[0];
    let domain_part = parts[1];
    
    // Check local part is not empty
    if local_part.is_empty() {
        return false;
    }
    
    // Check domain part has at least one dot and valid structure
    let domain_parts: Vec<&str> = domain_part.split('.').collect();
    if domain_parts.len() < 2 {
        return false;
    }
    
    // Check each domain part is not empty
    for part in domain_parts {
        if part.is_empty() {
            return false;
        }
    }
    
    // Basic character check (you can make this more sophisticated)
    if email.contains(' ') || email.contains("..") {
        return false;
    }
    
    true
}

// === HANDLERS ===

// Health check handler
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy"),
        (status = 503, description = "Service is unhealthy")
    )
)]
async fn health_check(service: web::Data<NetworkService>) -> HttpResponse {
    match service.health_check().await {
        Ok(_) => HttpResponse::Ok().json("Service is healthy"),
        Err(e) => {
            error!("Health check failed: {}", e);
            HttpResponse::ServiceUnavailable().json("Service is unhealthy")
        }
    }
}

// Pool stats handler
#[utoipa::path(
    get,
    path = "/api/v1/health/pool",
    responses(
        (status = 200, description = "Pool statistics", body = PoolStats),
        (status = 503, description = "Service is unhealthy")
    )
)]
async fn pool_stats(service: web::Data<NetworkService>) -> HttpResponse {
    match service.get_pool_stats().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => {
            error!("Failed to get pool stats: {}", e);
            HttpResponse::ServiceUnavailable().json("Service is unhealthy")
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/networks",
    responses(
        (status = 200, description = "List networks", body = [Network]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Network"
)]
async fn get_all_networks(service: web::Data<NetworkService>) -> Result<HttpResponse, NetworkError> {
    let networks = service.get_all().await?;
    Ok(HttpResponse::Ok().json(networks))
}

#[utoipa::path(
    get,
    path = "/api/v1/networks/{network_id}",
    params(
        ("network_id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Get network by id", body = Network),
        (status = 404, description = "Network not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Network"
)]
async fn get_network_by_id(
    network_id: web::Path<i32>,
    service: web::Data<NetworkService>,
) -> Result<HttpResponse, NetworkError> {
    let network = service.get_by_id(network_id.into_inner()).await?;
    match network {
        Some(network) => Ok(HttpResponse::Ok().json(network)),
        None => Err(NetworkError::NotFound),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/networks",
    request_body = NetworkCreate,
    responses(
        (status = 201, description = "Network created", body = Network),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Failed to create network")
    ),
    tag = "Network"
)]
async fn create_network(
    network: web::Json<NetworkCreate>,
    service: web::Data<NetworkService>,
) -> Result<HttpResponse, NetworkError> {
    // Validate the input data
    if let Some(ref email) = network.contact_email {
        if !is_valid_email(email) {
            return Err(NetworkError::Validation("Invalid email format".to_string()));
        }
    }
    
    let network = service.create(network.into_inner()).await?;
    Ok(HttpResponse::Created().json(network))
}

#[utoipa::path(
    put,
    path = "/api/v1/networks/{network_id}",
    request_body = NetworkUpdate,
    params(
        ("network_id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 200, description = "Network updated", body = Network),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Network not found"),
        (status = 500, description = "Failed to update network")
    ),
    tag = "Network"
)]
async fn update_network(
    network_id: web::Path<i32>,
    network: web::Json<NetworkUpdate>,
    service: web::Data<NetworkService>,
) -> Result<HttpResponse, NetworkError> {
    // Validate the input data
    if let Some(ref email) = network.contact_email {
        if !is_valid_email(email) {
            return Err(NetworkError::Validation("Invalid email format".to_string()));
        }
    }
    
    let network = service.update(network_id.into_inner(), network.into_inner()).await?;
    Ok(HttpResponse::Ok().json(network))
}

#[utoipa::path(
    delete,
    path = "/api/v1/networks/{network_id}",
    params(
        ("network_id" = i32, Path, description = "Network ID")
    ),
    responses(
        (status = 204, description = "Network deleted"),
        (status = 404, description = "Network not found"),
        (status = 500, description = "Failed to delete network")
    ),
    tag = "Network"
)]
async fn delete_network(
    network_id: web::Path<i32>,
    service: web::Data<NetworkService>,
) -> Result<HttpResponse, NetworkError> {
    service.delete(network_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

// === CONFIGURE ROUTES ===
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(API_PREFIX)
            .route("/health", web::get().to(health_check))
            .route("/health/pool", web::get().to(pool_stats))
            .route("/networks", web::get().to(get_all_networks))
            .route("/networks", web::post().to(create_network))
            .route("/networks/{network_id}", web::get().to(get_network_by_id))
            .route("/networks/{network_id}", web::put().to(update_network))
            .route("/networks/{network_id}", web::delete().to(delete_network)),
    );
}

// === OPENAPI DOC ===
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        pool_stats,
        get_all_networks,
        get_network_by_id,
        create_network,
        update_network,
        delete_network
    ),
    components(schemas(Network, NetworkCreate, NetworkUpdate, PoolStats)),
    tags(
        (name = "Network", description = "Network management endpoints")
    )
)]
struct ApiDoc;

// === MAIN ENTRY POINT ===
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/ev_db".to_string());

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();

    info!("Starting Configurator Service...");

    // Get max connections from environment variable
    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_MAX_CONNECTIONS);

    let service = match NetworkService::new_with_config(&database_url, max_connections).await {
        Ok(service) => service,
        Err(e) => {
            error!("Failed to create NetworkService: {}", e);
            panic!("Failed to create NetworkService: {}", e);
        }
    };

    let address = format!("{}:{}", server_host, server_port);
    info!("🚀 Server running on http://{}{}", address, API_PREFIX);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .configure(config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((server_host, server_port))?
    .run()
    .await
}