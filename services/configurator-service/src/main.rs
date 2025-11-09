use configurator_service::ConfiguratorApp;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set SERVICE_NAME environment variable safely
    // Using unsafe block for set_var since it's safe in this context
    unsafe {
        env::set_var("SERVICE_NAME", "configurator-service");
    }

    // Initialize logging from shared library
    shared::logger::init_logger();

    // Create app with config
    let app = match ConfiguratorApp::new() {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to create application: {}", e);
            eprintln!("Required environment variables:");
            eprintln!("  - PORT (e.g., 8080)");
            eprintln!("  - HOST (e.g., 127.0.0.1)");
            eprintln!("  - SERVICE_NAME (set in main.rs)");
            eprintln!("Optional environment variables:");
            eprintln!("  - DATABASE_URL (for services that need DB)");
            eprintln!("  - JWT_SECRET (for services that need auth)");
            eprintln!("  - CORS_ORIGINS (defaults to '*')");
            eprintln!("  - LOG_LEVEL (defaults to 'info')");
            std::process::exit(1);
        }
    };

    // Start the server
    app.run().await
}
