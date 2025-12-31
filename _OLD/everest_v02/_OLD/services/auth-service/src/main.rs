use actix_web::{web, App, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod application;
mod core;
mod domain;
mod infrastructure;
mod interfaces;
mod jobs;

use crate::core::{config::Config, database::init_db_pool};
use crate::interfaces::http::routes::configure_routes;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,auth_service=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;
    config.validate()?;

    tracing::info!(
        "Starting Auth Service on {}:{}",
        config.server_host,
        config.server_port
    );
    tracing::debug!("Keycloak URL: {}", config.keycloak_url);
    tracing::debug!("Keycloak Realm: {}", config.keycloak_realm);
    tracing::debug!("Backend Client ID: {}", config.keycloak_backend_client_id);

    // Initialize database pool
    let db_pool = init_db_pool(&config.database_url, config.database_max_connections).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    tracing::info!("Database migrations completed");

    // Initialize Keycloak client
    let keycloak_client = infrastructure::keycloak_client::KeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_auth_client_id.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
    );

    // Shared application state
    let app_state = web::Data::new(core::AppState {
        db_pool: db_pool.clone(),
        config: config.clone(),
        keycloak_client,
    });

    // Start cleanup job
    let cleanup_pool = db_pool.clone();
    tokio::spawn(async move {
        jobs::registration_cleanup_job::run_cleanup_job(cleanup_pool).await;
    });

    // Start HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure_routes)
    })
    .bind((config.server_host.as_str(), config.server_port))?
    .run();

    tracing::info!("Server started successfully");
    server.await?;

    Ok(())
}
