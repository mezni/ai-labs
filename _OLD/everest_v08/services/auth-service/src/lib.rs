use actix_web::{App, HttpServer, middleware::Logger, web};
use anyhow::Context;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Internal imports
pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use crate::core::{config::Config, database};
use crate::infrastructure::keycloak_client::HttpKeycloakClient;
use crate::presentation::controllers::{health_controller, registration_controller};
use crate::presentation::openapi::ApiDoc;

/// Shared application state accessible by all request handlers
pub struct AppState {
    pub config: Config,
    pub db_pool: sqlx::PgPool,
    pub keycloak_client: Arc<HttpKeycloakClient>,
}

pub async fn run() -> anyhow::Result<()> {
    // 1. Load Configuration
    let config = Config::from_env();

    // 2. Setup Database
    // Note: AppError from database::create_pool is converted to anyhow::Error here
    let db_pool = database::create_pool(&config.database_url)
        .await
        .context("Failed to initialize database pool")?;

    database::run_migrations(&db_pool)
        .await
        .context("Failed to run database migrations")?;

    database::check_connection(&db_pool)
        .await
        .context("Initial database health check failed")?;

    // 3. Initialize Keycloak Client
    let keycloak_client = Arc::new(HttpKeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
        config.keycloak_auth_client_id.clone(),
    ));

    // 4. Wrap App State for Actix
    let app_state = web::Data::new(AppState {
        config: config.clone(),
        db_pool,
        keycloak_client,
    });

    let openapi = ApiDoc::openapi();
    let server_addr = config.server_addr.clone();

    tracing::info!("🚀 Server starting at http://{}", server_addr);
    tracing::info!("📑 Swagger UI: http://{}/swagger-ui/", server_addr);

    // 5. Build and Run Server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Adds structured request logging
            .app_data(app_state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(
                web::scope("/api/v1")
                    .service(health_controller::health_check)
                    .configure(registration_controller::configure),
            )
    })
    .bind(&server_addr)
    .with_context(|| format!("Failed to bind server to {}", server_addr))?
    .run()
    .await
    .context("Server runtime error")?;

    Ok(())
}
