use actix_web::{middleware, App, HttpServer};
use auth_service::{
    core::{config::Config, database, logging},
    infrastructure::{cache::JwtKeyCache, keycloak::KeycloakClient, persistence::PostgresRepository},
    interfaces::routes,
};
use std::sync::Arc;
use tracing::info;

use auth_service::domain::repositories::UserRepository;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    logging::init();

    let config = Config::from_env()?;
    info!(
        "Starting Auth Service on {}:{}",
        config.server_host, config.server_port
    );

    let pool = database::create_pool(&config.database_url).await?;
//    database::run_migrations(&pool).await?;

    let keycloak = Arc::new(KeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
    ));

    let repository = Arc::new(PostgresRepository::new(pool.clone()));
    let jwt_cache = Arc::new(JwtKeyCache::new());

    // ---- CLONE CONFIG FOR ACTIX ----
    let config_data = actix_web::web::Data::new(config.clone());

    // Background sync job
    let sync_keycloak = keycloak.clone();
    let sync_repo = repository.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
            if let Err(e) = sync_users(&sync_keycloak, &sync_repo).await {
                tracing::error!("Background sync failed: {}", e);
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .app_data(actix_web::web::Data::new(repository.clone()))
            .app_data(actix_web::web::Data::new(keycloak.clone()))
            .app_data(actix_web::web::Data::new(jwt_cache.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(routes::configure)
    })
    .bind((config.server_host.as_str(), config.server_port))?
    .run()
    .await?;

    Ok(())
}


async fn sync_users(
    keycloak: &KeycloakClient,
    repo: &PostgresRepository,
) -> anyhow::Result<()> {
    info!("Starting user synchronization");
    let kc_users = keycloak.list_users().await?;
    
    for kc_user in kc_users {
        if let Some(user_id) = kc_user.attributes.as_ref()
            .and_then(|attrs| attrs.get("user_id"))
            .and_then(|v| v.first())
            .map(|s| s.to_string())
        {
            if let Ok(Some(mut db_user)) = repo.find_user_by_id(&user_id).await {
                let mut changed = false;
                if db_user.email != kc_user.email {
                    db_user.email = kc_user.email.clone();
                    changed = true;
                }
                if changed {
                    repo.update_user(&db_user).await?;
                    info!("Synced user {}", user_id);
                }
            }
        }
    }
    Ok(())
}