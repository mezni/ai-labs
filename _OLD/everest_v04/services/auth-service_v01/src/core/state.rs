use crate::core::{config::Config, database::DbPool};
use crate::infrastructure::keycloak::KeycloakClient;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub config: Config,
    pub keycloak_client: KeycloakClient,
}

impl AppState {
    pub fn new(db_pool: DbPool, config: Config, keycloak_client: KeycloakClient) -> Self {
        Self {
            db_pool,
            config,
            keycloak_client,
        }
    }
}