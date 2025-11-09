use tracing_subscriber::EnvFilter;

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
