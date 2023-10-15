use crate::auth::Session;
use crate::entities::{Profile, User};
use crate::store::Store;
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::collections::HashMap;

pub async fn login(State(store): State<Store>) -> Redirect {
    Redirect::permanent(&store.auth_client.login_url())
}

/// callback is the endpoint that GitHub redirects to after a successful login
/// It exchanges the code for a token and then fetches the user from GitHub.
/// If the user doesn't exist in the database, it creates a new user.
/// It then creates a session for the user and returns a JWT.
pub async fn callback(
    State(store): State<Store>,
    Query(params): Query<HashMap<String, String>>,
) -> Redirect {
    let code = match params.get("code") {
        Some(code) => code,
        None => return Redirect::to(&store.auth_client.post_auth_redirect_uri()),
    };

    let access_token = match store.auth_client.exchange_code(code.to_string()).await {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to exchange code: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri());
        }
    };

    let github_user = match store
        .auth_client
        .fetch_github_user(access_token.clone())
        .await
    {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Failed to fetch GitHub user: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri());
        }
    };

    let user = match User::upsert_from_github_user(&store.pool, github_user.clone()).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Failed to upsert user: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri());
        }
    };

    let user_id = match user.id.clone() {
        Some(id) => id,
        None => {
            eprintln!("User ID is None");
            return Redirect::to(&store.auth_client.post_auth_redirect_uri());
        }
    };

    let profile = match Profile::upsert_from_github_user(&store.pool, user_id, github_user).await {
        Ok(profile) => profile,
        Err(err) => {
            eprintln!("Failed to upsert profile: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri());
        }
    };

    let session = Session::new(user, profile, access_token.clone());

    let jwt = match encode(
        &Header::default(),
        &session,
        &EncodingKey::from_secret(access_token.secret().as_ref()), // QUESTION: Is this the right key?
    ) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to encode JWT: {}", err);
            return Redirect::to(&store.auth_client.post_auth_redirect_uri());
        }
    };

    Redirect::to(
        format!(
            "{}?token={}",
            &store.auth_client.post_auth_redirect_uri(),
            jwt
        )
        .as_str(),
    )
}
