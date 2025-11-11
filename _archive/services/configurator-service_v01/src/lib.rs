pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use shared::config::AppConfig;
use std::sync::Arc;

// Re-export main components for easy access
pub use api::{configure_docs, configure_routes};
pub use application::services::NetworkApplicationService;
pub use infrastructure::{Database, EventPublisher, PostgresNetworkRepository};

pub struct ConfiguratorApp {
    pub config: Arc<AppConfig>,
    pub app_service: NetworkApplicationService,
}

impl ConfiguratorApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = AppConfig::from_env()?;

        if config.database.url.is_empty() {
            return Err("DATABASE_URL is required for configurator service".into());
        }

        // Initialize infrastructure (database & repositories)
        let database =
            tokio::runtime::Handle::current().block_on(async { Database::new(&config).await })?;

        let network_repository = PostgresNetworkRepository::new(database.get_pool().clone());
        let event_publisher = EventPublisher::new();

        // Initialize application service
        let app_service =
            NetworkApplicationService::new(Box::new(network_repository), event_publisher);

        Ok(Self {
            config: Arc::new(config),
            app_service,
        })
    }

    fn get_server_address(&self) -> String {
        format!("{}:{}", self.config.server.host, self.config.server.port)
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let server_address = self.get_server_address();
        let app_service_data = web::Data::new(self.app_service.clone());

        println!(
            "🚀 Starting {} on {}",
            self.config.service_name, server_address
        );

        HttpServer::new(move || {
            let cors_origins = self.config.server.cors_origins.clone();
            let cors = if cors_origins.is_empty() || cors_origins.contains(&"*".to_string()) {
                Cors::permissive()
            } else {
                Cors::default()
                    .allowed_origin_fn(move |origin, _| {
                        cors_origins.iter().any(|o| o == origin.as_bytes())
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600)
            };

            App::new()
                .wrap(cors)
                .wrap(actix_web::middleware::Logger::default())
                .app_data(app_service_data.clone())
                .configure(|cfg| configure_routes(cfg, app_service_data.clone()))
                .configure(|cfg| configure_docs(cfg))
        })
        .bind(&server_address)?
        .run()
        .await
    }
}

// Clone implementation
impl Clone for ConfiguratorApp {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            app_service: self.app_service.clone(),
        }
    }
}
