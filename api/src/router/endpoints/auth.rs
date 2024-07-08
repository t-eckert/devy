use axum::{
    extract::{Query, State},
    http::header::{HeaderMap, LOCATION, SET_COOKIE},
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
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let jwt = match store
        .auth_client
        .clone()
        .handle_callback(&store.db, params)
        .await
    {
        Ok(token) => token,
        Err(err) => {
            let redirect_url = store
                .auth_client
                .clone()
                .redirect_url_with_err(&err.to_string());
            headers.insert(LOCATION, redirect_url.parse().unwrap());
            return (StatusCode::FOUND, headers);
        }
    };

    let cookie = Cookie::build(("token", jwt.clone()))
        .http_only(true)
        .same_site(SameSite::Lax);

    let cookie_header_value = format!("{}; Path=/; HttpOnly; SameSite=None; Secure", cookie);

    headers.insert(SET_COOKIE, cookie_header_value.parse().unwrap());
    headers.insert(
        LOCATION,
        store
            .auth_client
            .clone()
            .redirect_url_with_token(&jwt)
            .parse()
            .unwrap(),
    );

    (StatusCode::FOUND, headers)
}
