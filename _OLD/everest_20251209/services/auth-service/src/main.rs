mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::interfaces::api_doc::ApiDoc;
use crate::interfaces::routes;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env()?;

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Connected to database");

    // Create server
    let host = config.server_host.clone();
    let port = config.server_port;
    let config_data = actix_web::web::Data::new(config);

    tracing::info!("Starting auth service at {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(config_data.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(actix_web::web::scope("/api/v1").configure(routes::configure))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
