pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod api;
pub mod shared;

use actix_web::{App, HttpServer, middleware};
use actix_cors::Cors;
use shared::{config::AppConfig, logger, database};
use sqlx::PgPool;
use api::routes::init_routes;
use tracing::info;

pub async fn startup() -> std::io::Result<()> {
    // Load config from .env
    let config = AppConfig::from_env();

    // Initialize global logger with log level from .env
    logger::init(&config);

    info!("Starting configurator-service at {}:{}", config.host, config.port);

    // Create PostgreSQL connection pool
    let db_pool: PgPool = database::create_pg_pool(&config)
        .await
        .expect("Failed to create DB pool");

    info!("Database connected");

    // Configure CORS
    let cors = Cors::default()
        .allowed_origin_fn(move |origin, _req_head| {
            let origin_str = origin.to_str().unwrap_or_default();
            config.cors_allowed_origins.contains(&origin_str.to_string())
                || config.cors_allowed_origins.contains(&"*".to_string())
        })
        .allow_any_method()
        .allow_any_header()
        .supports_credentials();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(init_routes)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
