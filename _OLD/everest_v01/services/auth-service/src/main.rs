// src/main.rs
mod application;
mod core;
mod domain;
mod infrastructure;
mod interfaces;
mod jobs;

use crate::core::{Config, JwtValidator, create_pool, init_logging, run_migrations};
use crate::infrastructure::{
    KeycloakClient, PostgresAuditLogRepository, PostgresRegistrationRepository,
    PostgresUserRepository, TokenBlacklist,
};
use crate::interfaces::{ApiDoc, ServiceFactory, configure_routes};
use crate::jobs::KeycloakSyncJob;
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    init_logging();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    tracing::info!("Configuration loaded successfully");

    // Create database pool
    let pool = create_pool(&config.database)
        .await
        .expect("Failed to create database pool");
    tracing::info!("Database pool created");

    // Run migrations
    //    run_migrations(&pool)
    //        .await
    //        .expect("Failed to run migrations");
    //    tracing::info!("Database migrations completed");

    // Initialize shared dependencies
    let jwt_validator = Arc::new(JwtValidator::new(config.jwt.clone()));
    let keycloak_client = Arc::new(KeycloakClient::new(config.keycloak.clone()));
    let token_blacklist = Arc::new(TokenBlacklist::new_blacklist());

    // Initialize repositories
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()))
        as Arc<dyn domain::repositories::UserRepository>;
    let registration_repo = Arc::new(PostgresRegistrationRepository::new(pool.clone()))
        as Arc<dyn domain::repositories::RegistrationRepository>;
    let audit_repo = Arc::new(PostgresAuditLogRepository::new(pool.clone()))
        as Arc<dyn domain::repositories::AuditLogRepository>;

    // Initialize services
    let auth_service = ServiceFactory::create_auth_service(
        user_repo.clone(),
        registration_repo,
        audit_repo.clone(),
        keycloak_client.clone(),
        token_blacklist.clone(),
    );

    let user_service = ServiceFactory::create_user_service(
        user_repo.clone(),
        audit_repo.clone(),
        keycloak_client.clone(),
    );

    let audit_queries = ServiceFactory::create_audit_queries(audit_repo);

    // Initialize Keycloak sync job
    let sync_job = Arc::new(KeycloakSyncJob::new(
        pool.clone(),
        user_repo.clone(),
        keycloak_client.clone(),
        300, // Sync every 5 minutes
    ));

    // Start sync job in background
    let sync_job_handle = sync_job.clone();
    tokio::spawn(async move {
        sync_job_handle.start().await;
    });
    tracing::info!("Keycloak sync job started");

    // Server address
    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting server on {}", bind_address);

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let app = App::new()
            // Middleware
            .wrap(Logger::default())
            .wrap(cors)
            // Shared state
            .app_data(web::Data::new(jwt_validator.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(audit_queries.clone()))
            .app_data(web::Data::from(sync_job.clone()))
            // Swagger UI
            .service(
                SwaggerUi::new("/api/v1/swagger-ui/{_:.*}")
                    .url("/api/v1/openapi.json", ApiDoc::openapi()),
            )
            // Configure routes
            .configure(configure_routes);

        tracing::info!("Routes configured");
        app
    })
    .bind(&bind_address)?
    .run()
    .await
}
