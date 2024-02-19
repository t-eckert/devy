use auth::Provider;
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::collections::HashMap;
use store::Store;

pub fn make_router(store: Store<dyn Provider>) -> axum::Router<Store<dyn Provider>> {
    axum::Router::new()
        .route("/auth/login", get(login))
        .route("/auth/callback", get(callback))
        .with_state(store)
}

/// GET /auth/login
///
/// login is the endpoint that redirects the user to GitHub to login.
/// It returns a 308 redirect to GitHub's login page.
async fn login(State(store): State<Store<dyn Provider>>) -> Redirect {
    Redirect::permanent(&store.auth_client.login_url())
}

/// GET /auth/callback
///
/// callback is the endpoint that GitHub redirects to after a successful login
/// It exchanges the code for a token and then fetches the user from GitHub.
/// If the user doesn't exist in the database, it creates a new user.
/// It then creates a session for the user and returns a JWT.
async fn callback(
    State(store): State<Store<dyn Provider>>,
    Query(params): Query<HashMap<String, String>>,
) -> Redirect {
    // Get the code passed from GitHub in the query params.
    let code = match params.get("code") {
        Some(code) => code,
        None => {
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    let session = match store.auth_client.handle_callback(&store.pool, code).await {
        Ok(session) => session,
        Err(err) => {
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    let secret = auth::generate_encoding_key();

    // Encode the session as a JWT.
    let jwt = match encode(
        &Header::default(),
        &session,
        &EncodingKey::from_secret(secret.as_bytes()),
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
