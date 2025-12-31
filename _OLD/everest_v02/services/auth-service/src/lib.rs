pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use std::sync::Arc;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::core::config::Config;
use crate::core::database::create_pool;
use crate::domain::repositories::UserRegistrationRepository;
use crate::infrastructure::keycloak_client::KeycloakClient;
use crate::interfaces::http::routes::configure_routes;
use crate::interfaces::http::swagger::ApiDoc;

/// Global Application State
/// This struct holds shared resources like the DB pool and the Keycloak client.
pub struct AppState {
    pub db: sqlx::PgPool,
    pub env: Config,
    pub keycloak: KeycloakClient,
    // The Repository Trait Object
    pub user_registration_repo: Arc<dyn UserRegistrationRepository>,
}

pub async fn run() -> anyhow::Result<()> {
    // 1. Load configuration from environment variables
    let config = Config::from_env();

    // 2. Initialize Database Pool
    let pool = create_pool(&config.database_url).await?;

    // 3. Initialize Keycloak Infrastructure Client
    // We pass the whole config object as required by the implementation
    let keycloak_client = KeycloakClient::new(config.clone());

    let registration_repo = Arc::new(
        crate::infrastructure::persistence::user_registration_repo::PostgresUserRegistrationRepository::new(pool.clone())
    );

    // 3. Setup State
    let app_state = actix_web::web::Data::new(AppState {
        db: pool,
        env: config.clone(),
        keycloak: keycloak_client,
        user_registration_repo: registration_repo,
    });

    let server_addr = config.server_addr.clone();

    tracing::info!("Starting server on http://{}", server_addr);
    tracing::info!("Swagger UI: http://{}/swagger-ui/", server_addr);

    // 5. Start the HTTP Server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // Provide the state to all handlers
            .app_data(app_state.clone())
            // Middleware stacks
            .wrap(TracingLogger::default())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            // Register Swagger documentation UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            // Mount API routes
            .configure(configure_routes)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
