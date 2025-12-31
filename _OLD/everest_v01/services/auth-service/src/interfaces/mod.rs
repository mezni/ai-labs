// src/interfaces/mod.rs
pub mod admin_handlers;
pub mod audit_handlers;
pub mod auth_handlers;
pub mod health_handlers;
pub mod openapi;
pub mod routes;
pub mod sync_handlers;
pub mod user_handlers;

// Re-export commonly used items
pub use openapi::ApiDoc;
pub use routes::{ServiceFactory, configure_routes};
