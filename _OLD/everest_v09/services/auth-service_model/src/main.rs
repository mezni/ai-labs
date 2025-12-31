mod application;
mod core;
mod domain;
mod infrastructure;
mod presentation;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use core::{config::Config, database, logging};
use infrastructure::{
    keycloak_client::HttpKeycloakClient,
    repositories::{registration_repo::PgRegistrationRepository, user_repo::PgUserRepository},
};
use presentation::{
    controllers::{authentication_controller, health_controller, registration_controller},
    openapi::ApiDoc,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    logging::init_logging();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Create database pool
    let pool = database::create_pool(&config.database_url).await?;
    tracing::info!("Database connected");

    // Initialize repositories
    let user_repo = Arc::new(PgUserRepository::new(pool.clone()))
        as Arc<dyn domain::repositories::UserRepository>;
    let registration_repo = Arc::new(PgRegistrationRepository::new(pool.clone()))
        as Arc<dyn domain::repositories::RegistrationRepository>;

    // Initialize Keycloak client
    let keycloak = Arc::new(HttpKeycloakClient::new(
        config.keycloak.url.clone(),
        config.keycloak.realm.clone(),
        config.keycloak.backend_client_id.clone(),
        config.keycloak.backend_client_secret.clone(),
        config.keycloak.auth_client_id.clone(),
    )) as Arc<dyn infrastructure::keycloak_client::KeycloakClient>;

    // Initialize services
    let health_service = Arc::new(application::health_service::HealthService::new(
        pool.clone(),
    ));
    let registration_service =
        Arc::new(application::registration_service::RegistrationService::new(
            user_repo.clone(),
            registration_repo.clone(),
            keycloak.clone(),
        )) as Arc<dyn domain::services::RegistrationService>;
    let auth_service = Arc::new(
        application::authentication_service::AuthenticationService::new(
            user_repo.clone(),
            keycloak.clone(),
        ),
    ) as Arc<dyn domain::services::AuthenticationService>;

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting server on {}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(health_service.clone()))
            .app_data(web::Data::new(registration_service.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .service(
                web::scope("/api/v1")
                    .service(health_controller::health_check)
                    .service(registration_controller::register)
                    .service(registration_controller::verify)
                    .service(registration_controller::resend_verification)
                    .service(authentication_controller::login)
                    .service(authentication_controller::logout)
                    .service(authentication_controller::refresh_token),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
