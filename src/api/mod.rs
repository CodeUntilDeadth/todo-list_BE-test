mod controller;
mod docs;

use axum::{routing::get, Router};

use controller::{
    oauth2_gg::{login_by_gg, oauth_redirect},
    ping,
};
pub use docs::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn build() -> Router {
    let router = Router::new()
        .route("/", get(ping))
        .route("/auth/login", get(oauth_redirect))
        .route("/auth/login/callback", get(login_by_gg));
    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    router
}
