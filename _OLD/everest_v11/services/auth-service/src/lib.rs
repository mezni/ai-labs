pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;

use crate::core::{config::Config, database};
use crate::infrastructure::{
    keycloak_client::HttpKeycloakClient,
    repositories::{
        invitation_repo::PostgresInvitationRepository,
        registration_repo::PostgresRegistrationRepository, user_repo::PostgresUserRepository,
    },
};

use crate::application::{
    admin_service::AdminService, authentication_service::AuthenticationService,
    health_service::HealthService, invitation_service::InvitationService,
    registration_service::RegistrationService,
};

use crate::presentation::{
    controllers::{
        admin_controller, authentication_controller, health_controller, invitation_controller,
        registration_controller,
    },
    openapi::create_openapi_docs,
};

#[derive(Clone)]
pub struct AppState {
    pub health_service: Arc<HealthService>,
    pub registration_service: Arc<RegistrationService>,
    pub auth_service: Arc<AuthenticationService>,
    pub admin_service: Arc<AdminService>,
    pub invitation_service: Arc<InvitationService>,
}

impl AppState {
    pub async fn new(config: &Config) -> Self {
        // Database setup
        let pool = database::create_pool(&config.database_url)
            .await
            .expect("Failed to create database pool");

        database::run_migrations(&pool)
            .await
            .expect("Failed to run migrations");

        // Keycloak client
        let keycloak_client = Arc::new(HttpKeycloakClient::new(
            config.keycloak_url.clone(),
            config.keycloak_realm.clone(),
            config.keycloak_backend_client_id.clone(),
            config.keycloak_backend_client_secret.clone(),
            config.keycloak_auth_client_id.clone(),
        ));

        // Repositories
        let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
        let registration_repo = Arc::new(PostgresRegistrationRepository::new(pool.clone()));
        let invitation_repo = Arc::new(PostgresInvitationRepository::new(pool.clone()));

        // Services
        let health_service = Arc::new(HealthService::new(pool.clone()));
        let registration_service = Arc::new(RegistrationService::new(
            registration_repo.clone(),
            keycloak_client.clone(),
        ));
        let auth_service = Arc::new(AuthenticationService::new(
            user_repo.clone(),
            keycloak_client.clone(),
        ));
        let admin_service = Arc::new(AdminService::new(
            user_repo.clone(),
            keycloak_client.clone(),
        ));
        let invitation_service = Arc::new(InvitationService::new(
            invitation_repo.clone(),
            user_repo.clone(),
            keycloak_client.clone(),
        ));

        Self {
            health_service,
            registration_service,
            auth_service,
            admin_service,
            invitation_service,
        }
    }
}

pub async fn run() -> std::io::Result<()> {
    let config = Config::from_env();
    let app_state = AppState::new(&config).await;

    // OpenAPI documentation
    let openapi = create_openapi_docs();

    tracing::info!("Server starting on {}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(
                web::scope("/api/v1")
                    .configure(health_controller::configure)
                    .configure(registration_controller::configure)
                    .configure(authentication_controller::configure)
                    .configure(admin_controller::configure)
                    .configure(invitation_controller::configure),
            )
            .service(
                utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}