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
    Redirect::permanent(&store.auth_client.redirect_uri())
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
        None => return Redirect::to(&store.auth_client.redirect_uri()),
    };

    let access_token = match store.auth_client.exchange_code(code.to_string()).await {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to exchange code: {}", err);
            return Redirect::to(&store.auth_client.redirect_uri());
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
            return Redirect::to(&store.auth_client.redirect_uri());
        }
    };

    let user = User::upsert_from_github_user(&store.pool, github_user.clone()).await;
    let profile = Profile::upsert_from_github_user(&store.pool, github_user).await;

    let session = Session::new(user.unwrap(), profile.unwrap());

    let jwt = match encode(
        &Header::default(),
        &session,
        &EncodingKey::from_secret(access_token.secret().as_ref()),
    ) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to encode JWT: {}", err);
            return Redirect::to(&store.auth_client.redirect_uri());
        }
    };

    Redirect::to(format!("{}?token={}", &store.auth_client.redirect_uri(), jwt).as_str())
}
