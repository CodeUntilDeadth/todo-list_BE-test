mod api;
mod app_state;
mod config;
mod error;

use app_state::AppState;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
    dotenv().ok();
    let app_sate = AppState::new().await.unwrap();

    let listener = TcpListener::bind(format!("0.0.0.0:{}", *config::SERVER_PORT))
        .await
        .unwrap();

    info!("OpenAPI is opened!");

    info!(
        "Application is started and listening on port {:?}",
        *config::SERVER_PORT
    );

    axum::serve(listener, api::build(app_sate)).await.unwrap();
}
