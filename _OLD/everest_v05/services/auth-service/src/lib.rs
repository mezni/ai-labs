use actix_web::{App, HttpServer, middleware::Logger, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod application;
pub mod core;
pub mod infrastructure;
pub mod presentation;

use crate::core::{config::Config, database};
use crate::presentation::controllers::{
    admin_controller, authentication_controller, health_controller, registration_controller,
};
use crate::infrastructure::keycloak_client::HttpKeycloakClient;
use crate::presentation::openapi::ApiDoc;

pub struct AppState {
    pub config: Config,
    pub db_pool: database::DbPool,
    pub keycloak_client: Arc<HttpKeycloakClient>,
}

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env();
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

    let server_addr = config.server_addr.clone();
    let openapi = ApiDoc::openapi();

    tracing::info!("🚀 Server running at http://{}", server_addr);
    tracing::info!("📑 Swagger UI: http://{}/swagger-ui/", server_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(
                web::scope("/api/v1")
                    .service(health_controller::health_check)
                    .configure(registration_controller::configure)
                    .configure(authentication_controller::configure)
                    .configure(admin_controller::configure),
            )
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
