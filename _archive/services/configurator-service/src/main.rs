use configurator_service::startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    startup().await
}
