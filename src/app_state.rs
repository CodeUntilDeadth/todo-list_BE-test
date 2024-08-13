use dotenv::dotenv;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use crate::{
    config::{AppConfig, GoogleConfig},
    error::{Error, Result},
};

pub struct AppState {
    pub config: AppConfig,
    pub gg_client: BasicClient,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        dotenv().ok();

        let config = envy::from_env::<AppConfig>()?;
        let gg_config = envy::prefixed("GG_").from_env::<GoogleConfig>()?;

        let auth_url = AuthUrl::new(gg_config.clone().auth_url).map_err(|err| {
            let error = format!("Failed to parse auth_url [{}]: {}", gg_config.auth_url, err);
            Error::OAuthError(error)
        })?;

        let token_url = TokenUrl::new(gg_config.clone().token_url).map_err(|err| {
            let error = format!(
                "Failed to parse token_url [{}]: {}",
                gg_config.auth_url, err
            );
            Error::OAuthError(error)
        })?;

        let client = BasicClient::new(
            ClientId::new(gg_config.client_id),
            Some(ClientSecret::new(gg_config.client_secret)),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(gg_config.redirect_url)?);

        Ok(Self {
            config,
            gg_client: client,
        })
    }
}
