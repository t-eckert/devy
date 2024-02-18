use crate::store::Store;
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::collections::HashMap;
use tracing::error;

pub fn make_router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/auth/login", get(login))
        .route("/auth/callback", get(callback))
        .with_state(store)
}

/// GET /auth/login
///
/// login is the endpoint that redirects the user to GitHub to login.
/// It returns a 308 redirect to GitHub's login page.
async fn login(State(store): State<Store>) -> Redirect {
    Redirect::permanent(&store.auth_client.login_url())
}

/// GET /auth/callback
///
/// callback is the endpoint that GitHub redirects to after a successful login
/// It exchanges the code for a token and then fetches the user from GitHub.
/// If the user doesn't exist in the database, it creates a new user.
/// It then creates a session for the user and returns a JWT.
async fn callback(
    State(store): State<Store>,
    Query(params): Query<HashMap<String, String>>,
) -> Redirect {
    // Get the code passed from GitHub in the query params.
    let code = match params.get("code") {
        Some(code) => code,
        None => {
            error!("No code in params");
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    let session = match store.auth_client.handle_callback(&store.pool, code).await {
        Ok(session) => session,
        Err(err) => {
            error!("Failed to handle callback: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    // Encode the session as a JWT.
    let jwt = match encode(
        &Header::default(),
        &session,
        &EncodingKey::from_secret("85b07513-a8f8-45d9-88ae-4e2cbabe72a5".as_bytes()), // TODO generate this key. I don't use it to validate anything yet. Just a placeholder.
    ) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to encode JWT: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    // Redirect to the post-auth redirect URI with the JWT as a query param.
    Redirect::to(
        format!(
            "{}?token={}",
            &store.auth_client.post_auth_redirect_uri, jwt
        )
        .as_str(),
    )
}
