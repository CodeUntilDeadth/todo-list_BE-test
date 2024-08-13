use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GoogleConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub auth_url: String,
    pub token_url: String,
}
