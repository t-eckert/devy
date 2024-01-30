use crate::{
    auth::Session,
    entities::{profile, user},
    store::Store,
};
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

/// login is the endpoint that redirects the user to GitHub to login.
/// It returns a 308 redirect to GitHub's login page.
///
/// GET /auth/login
async fn login(State(store): State<Store>) -> Redirect {
    Redirect::permanent(&store.auth_client.login_url())
}

/// callback is the endpoint that GitHub redirects to after a successful login
/// It exchanges the code for a token and then fetches the user from GitHub.
/// If the user doesn't exist in the database, it creates a new user.
/// It then creates a session for the user and returns a JWT.
///
/// GET /auth/callback
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

    // Exchange the code for a token.
    let access_token = match store.auth_client.exchange_code(code.to_string()).await {
        Ok(token) => token,
        Err(err) => {
            error!("Failed to exchange code: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    // Fetch the GitHub user.
    let github_user = match store
        .auth_client
        .fetch_github_user(access_token.clone())
        .await
    {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to fetch GitHub user: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    // Upsert the user.
    let user = match user::upsert_from_github_user(&store.pool, github_user.clone()).await {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to upsert user: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    // Get the user ID.
    let user_id = user.id.clone().to_string();

    // Upsert the profile.
    let profile = match profile::upsert_from_github_user(&store.pool, user_id, github_user).await {
        Ok(profile) => profile,
        Err(err) => {
            error!("Failed to upsert profile: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri);
        }
    };

    // Create a session.
    let session = Session::new(user, profile, access_token.clone());

    // Encode the session as a JWT.
    let jwt = match encode(
        &Header::default(),
        &session,
        &EncodingKey::from_secret(access_token.secret().as_ref()), // QUESTION: Is this the right key?
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