pub mod config;
pub mod constants;
pub mod database;
pub mod errors;
// pub mod middleware;

use crate::infrastructure::keycloak_client::KeycloakClient;
use sqlx::PgPool;

pub struct AppState {
    pub db_pool: PgPool,
    pub config: config::Config,
    pub keycloak_client: KeycloakClient,
}
