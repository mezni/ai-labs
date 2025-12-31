// src/application/auth_services.rs
use crate::application::{auth_commands::AuthCommands, auth_queries::AuthQueries};
use crate::domain::repositories::{AuditLogRepository, RegistrationRepository, UserRepository};
use crate::infrastructure::{KeycloakClient, TokenBlacklist};
use std::sync::Arc;

/// Auth service facade combining commands and queries
#[derive(Clone)]
pub struct AuthService {
    pub commands: Arc<AuthCommands>,
    pub queries: Arc<AuthQueries>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        registration_repo: Arc<dyn RegistrationRepository>,
        audit_repo: Arc<dyn AuditLogRepository>,
        keycloak_client: Arc<KeycloakClient>,
        token_blacklist: Arc<TokenBlacklist>,
    ) -> Self {
        let commands = Arc::new(AuthCommands::new(
            user_repo.clone(),
            registration_repo,
            audit_repo,
            keycloak_client,
            token_blacklist.clone(),
        ));

        let queries = Arc::new(AuthQueries::new(user_repo, token_blacklist));

        Self { commands, queries }
    }
}