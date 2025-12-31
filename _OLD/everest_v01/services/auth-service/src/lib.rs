// src/lib.rs
pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;
pub mod jobs;

// Re-export commonly used items for easy access
pub use core::{Config, JwtValidator};
pub use infrastructure::{KeycloakClient, TokenBlacklist};
pub use interfaces::{ApiDoc, ServiceFactory, configure_routes};
pub use jobs::{KeycloakSyncJob, SyncStats};
