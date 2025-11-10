use configurator_service::ConfiguratorApp;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set SERVICE_NAME safely
    env::set_var("SERVICE_NAME", "configurator-service");

    // Initialize logger
    shared::logger::init_logger();

    // Create application
    let app = ConfiguratorApp::new().unwrap_or_else(|e| {
        eprintln!("Failed to create application: {}", e);
        eprintln!("Required environment variables:");
        eprintln!("  - DATABASE_URL");
        eprintln!("  - HOST");
        eprintln!("  - PORT");
        eprintln!("Optional:");
        eprintln!("  - JWT_SECRET");
        eprintln!("  - CORS_ORIGINS");
        eprintln!("  - LOG_LEVEL");
        std::process::exit(1);
    });

    // Run server
    app.run().await
}
