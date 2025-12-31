pub mod core;
pub mod domain;
pub mod infrastructure;

pub mod application {
    pub mod authentication_dto;
    pub mod authentication_service;
    pub mod health_dto;
    pub mod health_service;
    pub mod registration_dto;
    pub mod registration_service;
}
pub mod presentation {
    pub mod controllers {
        pub mod authentication_controller;
        pub mod health_controller;
        pub mod registration_controller;
    }
    pub mod openapi;
}

use crate::core::config::Config;
use crate::core::database;
use crate::infrastructure::keycloak_client::HttpKeycloakClient;
use actix_web::{App, HttpServer, middleware::Logger, web};
use sqlx::PgPool;
use std::sync::Arc;

use crate::domain::repositories::{RefreshTokenRepository, RegistrationRepository, UserRepository};
use crate::infrastructure::persistence::{
    RefreshTokenRepositoryImpl, RegistrationRepositoryImpl, UserRepositoryImpl,
};

pub struct AppState {
    pub config: Config,
    pub db_pool: PgPool,
    pub keycloak_client: Arc<HttpKeycloakClient>,
    // Add Repositories here
    pub user_repo: Arc<dyn UserRepository>,
    pub reg_repo: Arc<dyn RegistrationRepository>,
    pub token_repo: Arc<dyn RefreshTokenRepository>,
}

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env();
    let server_addr = config.server_addr.clone();

    let pool = database::create_pool(&config.database_url).await?;

    // Instantiate Repositories
    let user_repo = Arc::new(UserRepositoryImpl::new(pool.clone()));
    let reg_repo = Arc::new(RegistrationRepositoryImpl::new(pool.clone()));
    let token_repo = Arc::new(RefreshTokenRepositoryImpl::new(pool.clone()));

    let keycloak_client = Arc::new(HttpKeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
        config.keycloak_auth_client_id.clone(),
    ));

    let app_state = web::Data::new(AppState {
        config: config.clone(),
        db_pool: pool,
        keycloak_client,
        user_repo, // Injecting repos
        reg_repo,
        token_repo,
    });

    tracing::info!("🚀 Starting server on {}", server_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(presentation::controllers::health_controller::get_health)
            .configure(presentation::openapi::configure_openapi)
            .service(
                web::scope("/api/v1")
                    .configure(presentation::controllers::registration_controller::configure)
                    .configure(presentation::controllers::authentication_controller::configure),
            )
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
