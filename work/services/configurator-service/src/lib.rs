pub mod api;
pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used items for convenience
use actix_web::{App, HttpServer, web};
pub use api::docs::networks::ApiDoc;
pub use api::routes::config;
pub use application::services::networks::NetworkService;
pub use core::config::Config;
pub use core::database::ConnectionManager;
pub use core::logger::init_logging;
pub use core::middleware::{AuthenticatedUser, require_role};
pub use infrastructure::repositories::networks::NetworkRepository;
use utoipa_swagger_ui::SwaggerUi;

// Application builder for easier setup
pub struct ApplicationBuilder;

impl ApplicationBuilder {
    pub async fn create_service() -> Result<NetworkService, Box<dyn std::error::Error>> {
        let config = Config::from_env();

        let connection_manager =
            ConnectionManager::with_config(&config.database_url, config.db_max_connections).await?;

        // Test the connection
        sqlx::query("SELECT 1 FROM networks LIMIT 1")
            .execute(connection_manager.get_pool())
            .await?;

        let repository = NetworkRepository::new(std::sync::Arc::new(connection_manager));
        let service = NetworkService::new(repository);

        Ok(service)
    }

    pub async fn create_service_with_config(
        config: &Config,
    ) -> Result<NetworkService, Box<dyn std::error::Error>> {
        let connection_manager =
            ConnectionManager::with_config(&config.database_url, config.db_max_connections).await?;

        // Test the connection
        sqlx::query("SELECT 1 FROM networks LIMIT 1")
            .execute(connection_manager.get_pool())
            .await?;

        let repository = NetworkRepository::new(std::sync::Arc::new(connection_manager));
        let service = NetworkService::new(repository);

        Ok(service)
    }
}

// Utility functions
pub fn get_api_prefix() -> String {
    core::constants::API_PREFIX.to_string()
}

pub async fn run_server(service: NetworkService, address: &str) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .configure(config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::create_openapi()),
            )
    })
    .bind(address)?
    .run()
    .await
}
