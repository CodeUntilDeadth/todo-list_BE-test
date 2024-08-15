use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    app_state::AppState,
    error::{Error, Result},
    utils::gg_scopes::{GGScopeManager, ScopeKind},
};

#[derive(Debug, Deserialize)]
/// State is used when google call the callback function
/// Can be allow unsed temporarily
#[allow(unused)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn oauth_redirect(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client = &state.gg_client;

    let mut scope_manager = GGScopeManager::new();
    scope_manager.add(vec![ScopeKind::UserEmail, ScopeKind::UserProfile]);

    let scopes = scope_manager.get_all();

    // Generate the authorization URL with scopes.
    let (auth_url, _csrf_token) = client
        .authorize_url(|| oauth2::CsrfToken::new_random())
        .add_scopes(scopes.clone())
        .url(); // Redirect to Google's OAuth 2.0 server
    Redirect::temporary(&auth_url.to_string())
}

pub async fn login_by_gg(
    Query(params): Query<AuthRequest>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>> {
    let client = &state.gg_client;
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
            Err(Error::from(err))
        }
    }
}
