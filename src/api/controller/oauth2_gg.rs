use anyhow::Result;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::config;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn oauth_redirect() -> impl IntoResponse {
    // Initialize the OAuth2 client.
    let client = BasicClient::new(
        ClientId::new(config::GG_CLIENT_ID.to_string()),
        Some(ClientSecret::new(config::GG_CLIENT_SECRET.to_string())),
        AuthUrl::new(config::GG_AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(config::GG_TOKEN_URL.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(config::GG_REDIRECT_URI.to_string()).unwrap());

    // Add the desired scopes
    let scopes = vec![
        Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
        Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
    ];

    // Generate the authorization URL with scopes.
    let (auth_url, _csrf_token) = client
        .authorize_url(|| oauth2::CsrfToken::new_random())
        .add_scopes(scopes)
        .url(); // Redirect to Google's OAuth 2.0 server
    Redirect::temporary(&auth_url.to_string())
}

pub async fn login_by_gg(
    Query(params): Query<AuthRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Initialize the OAuth2 client.
    let client = BasicClient::new(
        ClientId::new(config::GG_CLIENT_ID.to_string()),
        Some(ClientSecret::new(config::GG_CLIENT_SECRET.to_string())),
        AuthUrl::new(config::GG_AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(config::GG_TOKEN_URL.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(config::GG_REDIRECT_URI.to_string()).unwrap());

    // Exchange the authorization code for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(async_http_client)
        .await;

    // Handle the result of the token exchange.
    match token_result {
        Ok(token) => {
            println!("Access token: {:?}", token);
            Ok(Json(json!({
                "status": "success",
                "access_token": token.access_token().secret()
            })))
        }
        Err(err) => {
            eprintln!("Error exchanging code for token: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to exchange authorization code for token: {}", err),
            ))
        }
    }
}
