// src/application/user_services.rs
use crate::application::{user_commands::UserCommands, user_queries::UserQueries};
use crate::domain::repositories::{AuditLogRepository, UserRepository};
use crate::infrastructure::KeycloakClient;
use std::sync::Arc;

/// User service facade combining commands and queries
#[derive(Clone)]
pub struct UserService {
    pub commands: Arc<UserCommands>,
    pub queries: Arc<UserQueries>,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        audit_repo: Arc<dyn AuditLogRepository>,
        keycloak_client: Arc<KeycloakClient>,
    ) -> Self {
        let commands = Arc::new(UserCommands::new(
            user_repo.clone(),
            audit_repo,
            keycloak_client,
        ));

        let queries = Arc::new(UserQueries::new(user_repo));

        Self { commands, queries }
    }
}
