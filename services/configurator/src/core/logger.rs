use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging(log_level: &str) {
    fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();
    
    tracing::info!("Logging initialized with level: {}", log_level);
}