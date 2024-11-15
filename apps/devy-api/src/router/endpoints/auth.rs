use crate::store::Store;
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
};
use std::collections::HashMap;

/// Create a new router for auth.
pub fn router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/auth/sign-in", get(sign_in))
        .route("/auth/callback", get(callback))
        .with_state(store)
}

// GET /auth/sign-in
async fn sign_in(State(store): State<Store>) -> Redirect {
    Redirect::to(&store.auth_client.login())
}

// GET /auth/callback
async fn callback(
    State(store): State<Store>,
    Query(params): Query<HashMap<String, String>>,
) -> Redirect {
    match store
        .auth_client
        .clone()
        .handle_callback(&store.db_conn, params)
        .await
    {
        Ok(token) => Redirect::to(&store.auth_client.redirect_url_with_token(&token)),
        Err(err) => Redirect::to(&store.auth_client.redirect_url_with_err(&err.to_string())),
    }
}
