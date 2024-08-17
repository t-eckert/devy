use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use cookie::{Cookie, SameSite};
use lib::store::Store;
use std::collections::HashMap;

/// Create a new router for Auth.
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
        .handle_callback(&store.db, params)
        .await
    {
        Ok(token) => {
            Redirect::to(&store
                .auth_client
                .redirect_url_with_token(&token))
        },
        Err(err) => {
            Redirect::to(&store
                .auth_client
                .redirect_url_with_err(&err.to_string()))
        }
    }
}
