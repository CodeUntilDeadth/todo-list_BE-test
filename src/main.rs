mod api;
mod app_state;
mod config;
mod error;
mod model;

use std::sync::Arc;

use app_state::AppState;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .with(ErrorLayer::default())
        .init();

    let app_sate = Arc::new(AppState::new().await.unwrap());
    let config = &app_sate.clone().config;
    let server = api::build(app_sate);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
        .await
        .unwrap();

    info!("OpenAPI is opened!");

    info!(
        "Application is started and listening on port {:?}!!!",
        config.server_port
    );

    axum::serve(listener, server).await.unwrap();
}
