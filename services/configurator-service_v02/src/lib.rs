pub mod core;
pub mod api;
pub mod application;
pub mod domain;

pub use core::{AppConfig, AppError, init_logger};
pub use core::{config, errors, logger};

use actix_web::{App, HttpServer, Responder, middleware, web};
use tracing::{info, instrument};
use std::sync::Arc;
use crate::api::networks::{
    create_network_handler, get_network_by_id_handler, get_all_networks_handler,
    update_network_handler, delete_network_handler,
};

#[derive(Clone)]
pub struct ServiceApp<R: domain::repositories::network_repository::NetworkRepository + Send + Sync + 'static> {
    config: AppConfig,
    network_service: Arc<application::services::network_application_service::NetworkApplicationService<R>>,
}

impl<R: domain::repositories::network_repository::NetworkRepository + Send + Sync + 'static> ServiceApp<R> {
    pub fn new(repo: R) -> Result<Self, AppError> {
        let config = AppConfig::new()?;
        let network_service = Arc::new(application::services::network_application_service::NetworkApplicationService::new(repo));
        Ok(ServiceApp { config, network_service })
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

    pub async fn run(&self) -> Result<(), AppError> {
        let addr = self.config.server_address();
        info!("Server starting on {}", addr);

        let network_service_data = web::Data::new(self.network_service.clone());

        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(middleware::Compress::default())
                .route("/", web::get().to(Self::index))
                .route("/health", web::get().to(Self::health))
                .app_data(network_service_data.clone())
                // Network API routes
                .service(create_network_handler)
                .service(get_network_by_id_handler)
                .service(get_all_networks_handler)
                .service(update_network_handler)
                .service(delete_network_handler)
        })
        .bind(&addr)?
        .shutdown_timeout(30)
        .run()
        .await?;

        Ok(())
    }
}
