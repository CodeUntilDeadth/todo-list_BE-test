mod controller;
mod docs;

use axum::{routing::get, Router};

use controller::ping;
pub use docs::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn build() -> Router {
    let router = Router::new().route("/", get(ping));

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    router
}
