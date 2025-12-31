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
use crate::infrastructure::keycloak_client::HttpKeycloakClient;
use presentation::{controllers::health_controller, openapi::ApiDoc};

pub struct AppState {
    pub config: Config,
    pub db_pool: sqlx::PgPool,
    pub keycloak_client: Arc<HttpKeycloakClient>,
}

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env();

    // Create database pool
    let db_pool = database::create_pool(&config.database_url).await?;

    let keycloak_client = Arc::new(HttpKeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
        config.keycloak_auth_client_id.clone(),
    ));

    let app_state = web::Data::new(AppState {
        config: config.clone(),
        db_pool,
        keycloak_client,
    });

    let openapi = ApiDoc::openapi();
    let server_addr = config.server_addr.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(app_state.clone())
            // Register Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(web::scope("/api/v1").service(health_controller::health_check))
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
