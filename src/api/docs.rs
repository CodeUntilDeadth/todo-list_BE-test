use crate::model::account::Account;

use super::controller;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(controller::ping, controller::users::get_all),
    components(schemas(Account))
)]
pub struct ApiDoc;
