use dotenvy::dotenv;
use std::env;
use tracing_subscriber::{EnvFilter, fmt};

/// Initializes a tracing-based logger.
/// Automatically reads LOG_LEVEL from .env or defaults to "info".
pub fn init_logger() {
    dotenv().ok();

    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    // Configure tracing subscriber
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&log_level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false) // hide target module names
        .with_level(true)
        .with_thread_ids(false)
        .with_line_number(true)
        .compact() // shorter format
        .init();

    tracing::info!("🪵 Logger initialized with level: {}", log_level);
}
