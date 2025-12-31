// src/core/mod.rs
pub mod config;
pub mod constants;
pub mod database;
pub mod errors;
pub mod id_generator;
pub mod jwt;
pub mod logging;
pub mod middleware;

// Re-export commonly used items
pub use config::Config;
pub use constants::*;
pub use database::{DbTransaction, create_pool, run_migrations};
pub use errors::{AppError, AppResult};
pub use id_generator::IdGenerator;
pub use jwt::{Claims, JwtValidator};
pub use logging::init_logging;
pub use middleware::{JwtAuth, RequireRole, extract_claims};
