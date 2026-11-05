use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn health() -> &'static str {
    "bio-com API is running"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new().route("/health", get(health));

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind API listener");

    tracing::info!("bio-com API listening on http://127.0.0.1:8080");

    axum::serve(listener, app).await.expect("API server failed");
}
