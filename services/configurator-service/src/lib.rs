use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, http, web};
use serde::Serialize;
use shared::config::AppConfig;
use std::sync::Arc;

pub struct ConfiguratorApp {
    pub config: Arc<AppConfig>,
}

impl ConfiguratorApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = AppConfig::from_env()?;

        // Log which optional features are configured
        if config.database.url.is_empty() {
            println!("ℹ️  Database URL not set - this service doesn't need DB");
        }

        if config.auth.jwt_secret.is_empty() {
            println!("ℹ️  JWT secret not set - this service doesn't need auth");
        }

        Ok(Self {
            config: Arc::new(config),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.config.server.host, self.config.server.port)
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let server_address = self.get_server_address();
        let config_data = web::Data::new(self.config.clone());

        println!(
            "🚀 Starting {} on {}",
            self.config.service_name, server_address
        );
        println!("📋 Service info:");
        println!("   - Environment: {}", self.config.environment);
        println!("   - Host: {}", self.config.server.host);
        println!("   - Port: {}", self.config.server.port);
        println!("   - CORS: {:?}", self.config.server.cors_origins);
        println!("   - Log Level: {}", self.config.logging.level);

        HttpServer::new(move || {
            // Create CORS configuration fresh for each worker
            let cors_origins = config_data.server.cors_origins.clone();
            let cors = if cors_origins.is_empty() || cors_origins.contains(&"*".to_string()) {
                Cors::permissive()
            } else {
                Cors::default()
                    .allowed_origin_fn(move |origin, _req_head| {
                        let origin_str = origin.to_str().unwrap_or("");
                        cors_origins.iter().any(|allowed| allowed == origin_str)
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600)
            };

            App::new()
                .wrap(cors)
                .app_data(config_data.clone())
                .route("/api/v1/health", web::get().to(health_check))
                .route("/api/v1/config", web::get().to(get_config))
                .route("/api/v1/info", web::get().to(get_server_info))
        })
        .bind(&server_address)?
        .run()
        .await
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(shared::response::ApiResponse::success("Service is healthy"))
}

async fn get_config(config: web::Data<Arc<AppConfig>>) -> impl Responder {
    HttpResponse::Ok().json(shared::response::ApiResponse::success(&*config))
}

async fn get_server_info(config: web::Data<Arc<AppConfig>>) -> impl Responder {
    let info = ServerInfo {
        service_name: config.service_name.clone(),
        environment: config.environment.clone(),
        server_address: format!("{}:{}", config.server.host, config.server.port),
        log_level: config.logging.level.clone(),
        cors_origins: config.server.cors_origins.clone(),
        has_database: !config.database.url.is_empty(),
        has_jwt_secret: !config.auth.jwt_secret.is_empty(),
    };
    HttpResponse::Ok().json(shared::response::ApiResponse::success(info))
}

#[derive(Serialize)]
struct ServerInfo {
    service_name: String,
    environment: String,
    server_address: String,
    log_level: String,
    cors_origins: Vec<String>,
    has_database: bool,
    has_jwt_secret: bool,
}
