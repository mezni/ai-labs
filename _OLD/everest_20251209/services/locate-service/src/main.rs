mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;
mod middleware;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use application::{ReviewService, StationService};
use config::Config;
use infrastructure::{ReviewRepository, StationRepository};
use interfaces::api_doc::ApiDoc;
use middleware::JwtAuth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "locate_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connection established");

    // Create repositories
    let station_repository = StationRepository::new(pool.clone());
    let review_repository = ReviewRepository::new(pool.clone());

    // Create services
    let station_service = web::Data::new(StationService::new(station_repository));
    let review_service = web::Data::new(ReviewService::new(review_repository));

    // Create JWT middleware
    let jwt_auth = JwtAuth::new(config.jwks_url.clone(), config.jwt_issuer.clone());

    // Create OpenAPI documentation
    let openapi = ApiDoc::openapi();

    let server_address = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("Starting server at http://{}", server_address);
    tracing::info!(
        "Swagger UI available at http://{}/swagger-ui/",
        server_address
    );

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(station_service.clone())
            .app_data(review_service.clone())
            .configure(|cfg| interfaces::routes::configure_routes(cfg, jwt_auth.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(&server_address)?
    .run()
    .await
}
