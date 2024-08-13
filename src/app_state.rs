use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use crate::{config, error::Result};

#[derive(Clone)]
pub struct AppState {
    pub gg_client: BasicClient,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let auth_url = AuthUrl::new(config::GG_AUTH_URL.to_string())
            .map_err(|err| {
                format!(
                    "Failed to parse auth_url [{}]: {}",
                    config::GG_AUTH_URL.to_string(),
                    err
                )
            })
            .unwrap();
        let token_url = TokenUrl::new(config::GG_TOKEN_URL.to_string())
            .map_err(|err| {
                format!(
                    "Failed to parse auth_url [{}]: {}",
                    config::GG_TOKEN_URL.to_string(),
                    err
                )
            })
            .unwrap();

        let client = BasicClient::new(
            ClientId::new(config::GG_CLIENT_ID.to_string()),
            Some(ClientSecret::new(config::GG_CLIENT_SECRET.to_string())),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(config::GG_REDIRECT_URI.to_string()).unwrap());

        Ok(Self { gg_client: client })
    }
}
