use bson::doc;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use tracing::info;

use crate::{
    config::{AppConfig, GoogleConfig},
    error::{Error, Result},
};

pub struct AppState {
    pub config: AppConfig,
    pub gg_client: BasicClient,
    pub db: Database,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        dotenv().ok();

        let config = envy::from_env::<AppConfig>()?;
        let gg_client = init_gg_client()?;

        let db = init_db(&config.database.url, &config.database.name).await?;

        info!("Connection to database successfully!");

        Ok(Self {
            config,
            gg_client,
            db,
        })
    }
}

fn init_gg_client() -> Result<BasicClient> {
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

    Ok(BasicClient::new(
        ClientId::new(gg_config.client_id),
        Some(ClientSecret::new(gg_config.client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(gg_config.redirect_url)?))
}

async fn init_db(db_url: &str, db_name: &str) -> Result<Database> {
    let options = ClientOptions::parse(db_url).await?;
    let client = Client::with_options(options)?;
    let db = client.database(db_name);

    // Ping server if it's connected
    let _ = db
        .run_command(doc! {"ping": 1})
        .await
        .map_err(|err| Error::Other(err.into()));

    Ok(db)
}
