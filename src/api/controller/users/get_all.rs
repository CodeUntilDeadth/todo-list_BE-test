use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{app_state::AppState, error::Result, model::account::Account};

#[utoipa::path(
    get,
    tag = "User",
    path = "/users",
    responses (
            (status = 200, body = Account)
        )
    )
]
pub async fn get_all(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Account>>> {
    let accounts = Account::get_all(&state.db).await?;

    Ok(Json(accounts))
}
