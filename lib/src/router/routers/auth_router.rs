use crate::store::Store;
use axum::{
    extract::{Query, State},
    http::header::{HeaderMap, LOCATION, SET_COOKIE},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use cookie::{Cookie, SameSite};
use std::collections::HashMap;

/// `/auth/*`
///
/// Router for handling authentication requests.
pub struct AuthRouter;

impl AuthRouter {
    /// Create a new AuthRouter.
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/auth/public-key", get(public_key))
            .route("/auth/login", get(login))
            .route("/auth/callback", get(callback))
            .with_state(store)
    }
}

/// `GET /auth/public-key`
///
/// Returns the public key of the encoding JWT.
async fn public_key(State(store): State<Store>) -> impl IntoResponse {
    let public_key = store.auth_client.public_key();
    (StatusCode::OK, public_key)
}

/// `GET /auth/login`
///
/// Redirects the user to the login page.
async fn login(State(store): State<Store>) -> Redirect {
    Redirect::to(&store.auth_client.login())
}

/// GET /auth/callback
///
/// Handles the callback of the auth provider and redirect the user to the correct page.
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
