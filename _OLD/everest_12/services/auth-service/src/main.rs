use anyhow::Context;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    use auth_service::core::{config::Config, logging};

    let config = Config::from_env();

    logging::init_logging(&config.log_level);

    auth_service::run()
        .await
        .context("Application startup failed")?;

    Ok(())
}
