pub mod core;

// Re-export core modules at the crate level for easy access
pub use core::{AppConfig, ServiceError, init_logger};
pub use core::{config, errors, logger}; // Remove LoggerConfig

use actix_web::{App, HttpServer, Responder, middleware, web};
use tracing::{info, instrument};

#[derive(Clone)]
pub struct ServiceApp {
    config: AppConfig,
}

impl ServiceApp {
    pub fn new() -> Result<Self, ServiceError> {
        let config = AppConfig::new()?;
        Ok(ServiceApp { config })
    }

    #[instrument]
    async fn index() -> impl Responder {
        info!("Index route called");
        "Hello, World!"
    }

    #[instrument]
    async fn health() -> impl Responder {
        "OK"
    }

    pub async fn run(&self) -> Result<(), ServiceError> {
        let addr = self.config.server_address();
        info!("Server starting on {}", addr);

        HttpServer::new(|| {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(middleware::Compress::default())
                .route("/", web::get().to(Self::index))
                .route("/health", web::get().to(Self::health))
        })
        .bind(&addr)?
        .shutdown_timeout(30)
        .run()
        .await?;

        Ok(())
    }
}
