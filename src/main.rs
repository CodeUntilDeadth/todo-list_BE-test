mod api;
mod app_state;
mod config;
mod error;

use std::sync::Arc;

use app_state::AppState;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let app_sate = Arc::new(AppState::new().await.unwrap());
    let config = &app_sate.clone().config;
    let server = api::build(app_sate);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
        .await
        .unwrap();

    info!("OpenAPI is opened!");

    info!(
        "Application is started and listening on port {:?}",
        config.server_port
    );

    axum::serve(listener, server).await.unwrap();
}
