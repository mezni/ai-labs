use actix_web::{web, App, HttpServer};
use auth_service::core::{config::Config, database, logging, state::AppState};
use auth_service::presentation::openapi::create_openapi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    logging::init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    tracing::info!("Configuration loaded successfully");

    // Initialize database pool
    let db_pool = database::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");
    tracing::info!("Database pool created successfully");

    // Create Keycloak client
    let keycloak_client = auth_service::infrastructure::keycloak::KeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
    );

    // Create app state
    let app_state = AppState::new(db_pool, config.clone(), keycloak_client);

    let server_host = config.server_host.clone();
    let server_port = config.server_port;

    tracing::info!("Starting server at {}:{}", server_host, server_port);

    HttpServer::new(move || {
        let openapi = create_openapi();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(auth_service::presentation::controllers::configure_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((server_host, server_port))?
    .run()
    .await
}