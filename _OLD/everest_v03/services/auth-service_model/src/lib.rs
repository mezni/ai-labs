pub mod application;
pub mod core;
pub mod interfaces;

use crate::core::config::Config;
use crate::core::database::create_pool;
use crate::core::state::AppState;
use crate::interfaces::http::configure_routes;
use crate::interfaces::swagger::configure_swagger;

use actix_web::{App, HttpServer, middleware::Logger, web};

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await?;

    let app_state = web::Data::new(AppState::new(config.clone(), pool));
    let server_addr = config.server_addr.clone();

    tracing::info!("Starting Actix server on {}", server_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            // Register API routes
            .configure(configure_routes)
            // Register Swagger UI
            .configure(configure_swagger)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
