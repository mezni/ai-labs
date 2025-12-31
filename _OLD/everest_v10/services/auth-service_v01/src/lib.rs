use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
use crate::core::{config::Config, database};
use crate::domain::repositories::{RegistrationRepository, UserRepository};
use crate::infrastructure::keycloak_client::HttpKeycloakClient;
use crate::infrastructure::repositories::{
    registration_repo::PgRegistrationRepository, user_repo::PgUserRepository,
};
use presentation::{
    controllers::admin_controller, controllers::authentication_controller,
    controllers::health_controller, controllers::registration_controller, openapi::ApiDoc,
};

pub struct AppState {
    pub config: Config,
    pub db_pool: sqlx::PgPool,
    pub keycloak_client: Arc<HttpKeycloakClient>,
    pub user_repo: Arc<dyn UserRepository>,
    pub registration_repo: Arc<dyn RegistrationRepository>,
}

pub async fn run() -> anyhow::Result<()> {

    
    let config = Config::from_env();

    // 1. Fixed variable name: changed 'pool' to 'db_pool'
    let db_pool = database::create_pool(&config.database_url).await?;

    // 2. Pass db_pool to repositories
    let user_repo = Arc::new(PgUserRepository::new(db_pool.clone()));
    let registration_repo = Arc::new(PgRegistrationRepository::new(db_pool.clone()));

    let keycloak_client = Arc::new(HttpKeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
        config.keycloak_auth_client_id.clone(),
    ));

    // Wrap in web::Data for Actix
    let app_state = web::Data::new(AppState {
        config: config.clone(),
        db_pool,
        keycloak_client,
        user_repo,
        registration_repo,
    });

    
    let openapi = ApiDoc::openapi();
    let server_addr = config.server_addr.clone();

    println!("Starting server at http://{}", server_addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(app_state.clone()) // This injects AppState into your handlers
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(
                web::scope("/api/v1")
                    .configure(health_controller::configure)
                    .configure(registration_controller::configure)
                    .configure(authentication_controller::configure)
                    .configure(admin_controller::configure_admin),
            )
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
