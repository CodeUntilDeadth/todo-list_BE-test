mod controller;
mod docs;

use std::sync::Arc;

use axum::{routing::get, Router};

use controller::{
    auth::oauth2_gg::{login_by_gg, oauth_redirect},
    ping,
    users::get_all,
};
pub use docs::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::app_state::AppState;

pub fn build(state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/", get(ping))
        .route("/auth/login", get(oauth_redirect))
        .route("/auth/login/callback", get(login_by_gg))
        .route("/users", get(get_all));

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    router.with_state(state)
}
