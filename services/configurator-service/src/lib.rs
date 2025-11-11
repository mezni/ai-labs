pub mod api;
pub mod core;

use actix_web::{App, HttpServer, web};
use api::openapi::ApiDoc;
use core::{AppConfig, Database, logger};
use std::io;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi; // Add this import

/// Builds and configures the Actix Web application
pub fn build_app(
    db: Database,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(db))
        .configure(api::routes::configure_routes)
        // Swagger UI available at /swagger-ui
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/swagger-ui/openapi.json", ApiDoc::openapi()), // This should work now
        )
}

/// Starts the HTTP server using .env configuration
pub async fn start_server() -> io::Result<()> {
    dotenvy::dotenv().ok();

    // Load configuration and initialize logger
    let config = AppConfig::new().expect("Failed to load configuration");
    logger::init_logger();

    // Initialize database
    let db = Database::new(&config)
        .await
        .expect("Failed to connect to database");

    let address = config.server_address();

    tracing::info!("🚀 Starting server at http://{}", address);

    HttpServer::new(move || build_app(db.clone()))
        .bind(address)?
        .run()
        .await
}
