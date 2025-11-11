use crate::shared::config::AppConfig;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init(config: &AppConfig) {
    let log_level = &config.log_level;
    let filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}
