// main.rs
use configurator_service::{ApplicationBuilder, Config, init_logging};
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    init_logging(&config.log_level);

    info!("Starting Configurator Service...");

    let service = ApplicationBuilder::create_service_with_config(&config)
        .await
        .expect("Failed to create service");

    let address = config.server_address();
    info!(
        "🚀 Server running on http://{}{}",
        address,
        configurator_service::get_api_prefix()
    );
    info!("📚 Swagger UI available on http://{}/swagger-ui/", address);
    info!("🔐 JWT Authentication is enabled");

    configurator_service::run_server(service, &address).await
}
