use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logging(log_level: &str) {
    fmt().with_env_filter(EnvFilter::new(log_level)).init();

    tracing::info!("Logging initialized with level: {}", log_level);
}
