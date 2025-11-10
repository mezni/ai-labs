use configurator_service::ServiceApp;
use dotenvy::dotenv;
use tracing::{error, info};

const SERVICE_NAME: &str = "configurator-service";

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Initialize logger first
    configurator_service::logger::init_logger()?;

    // Create app after logger is initialized
    let app = match ServiceApp::new() {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to create application: {}", e);
            std::process::exit(1);
        }
    };

    info!("{} starting", SERVICE_NAME);
    
    if let Err(e) = app.run().await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }

    info!("{} stopped gracefully", SERVICE_NAME);
    Ok(())
}