use anyhow::Context;
use auth_service::core::{config::Config, logging};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load config first to get the log level
    let config = Config::from_env();

    // 2. Initialize logging with the level from environment
    logging::init_logging(&config.log_level);

    tracing::info!("Log level set to: {}", config.log_level);

    // 3. Start application
    auth_service::run()
        .await
        .context("Application execution failed")?;

    Ok(())
}
