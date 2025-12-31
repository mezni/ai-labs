// src/infrastructure/mod.rs
pub mod audit_repo;
pub mod cache;
pub mod keycloak_client;
pub mod registration_repo;
pub mod user_repo;

// Re-export commonly used items
pub use audit_repo::PostgresAuditLogRepository;
pub use cache::{Cache, CachedUser, TokenBlacklist, UserCache};
pub use keycloak_client::{CreateUserRequest, KeycloakClient, KeycloakUser, UserCredential};
pub use registration_repo::PostgresRegistrationRepository;
pub use user_repo::PostgresUserRepository;
