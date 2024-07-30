mod controller;

use axum::{routing::get, Router};

use controller::ping;

pub fn build() -> Router {
    let routes = Router::new().route("/", get(ping));

    routes
}
