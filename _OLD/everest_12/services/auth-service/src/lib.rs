use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

pub mod core;
pub mod domain;

use crate::core::{
    config::Config,
    database::{self, DbPool},
    errors::AppError,
};

pub struct AppState {
    pub db: DbPool,
}

pub async fn run() -> Result<(), AppError> {
    let config = Config::from_env();

    let db_pool = database::create_pool(&config.database_url).await?;

    let app_state = web::Data::new(AppState { db: db_pool });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(app_state.clone())
    })
    .bind(&config.server_addr)
    .map_err(|e| AppError::Internal(e.to_string()))?
    .run()
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(())
}
