use utoipa::OpenApi;

use super::controller;
#[derive(OpenApi)]
#[openapi(paths(controller::ping), components())]
pub struct ApiDoc;
