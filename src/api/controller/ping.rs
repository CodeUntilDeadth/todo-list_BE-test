/// Use to test if server is running
#[utoipa::path(
    get,
    tag = "Ping",
    path = "/",
    responses (
        (status = 200, description = "Pong!")
    )
)]
pub async fn ping() -> &'static str {
    "Pong!!!!!!!"
}
