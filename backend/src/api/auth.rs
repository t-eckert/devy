use crate::auth;
use crate::db::DB;
use jsonwebtoken::{encode, EncodingKey, Header};
use oauth2::reqwest::async_http_client;
use oauth2::{basic::BasicClient, AuthorizationCode, ClientId, CsrfToken};
use oauth2::{AuthUrl, ClientSecret, Scope, TokenResponse, TokenUrl};
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

pub fn routes() -> Vec<rocket::Route> {
    routes![login, logout, callback]
}

/// login redirects the user to GitHub's auth page
#[get("/login?<redirect>")]
async fn login(redirect: Option<String>) -> Redirect {
    let auth_url = auth::get_uri().unwrap();
    Redirect::to(auth_url.clone().to_string())
}

/// TODO logout should clear the session
#[get("/logout")]
async fn logout() -> Redirect {
    Redirect::to(uri!("http://localhost:3000"))
}

/// callback is the endpoint that GitHub redirects to after a successful login
/// It exchanges the code for a token and then fetches the user from GitHub.
/// If the user doesn't exist in the database, it creates a new user.
/// It then creates a session for the user and returns a JWT.
#[get("/callback?<code>")]
async fn callback(mut conn: Connection<DB>, code: Option<String>) -> Redirect {
    // Short circuit if we don't have a code.
    let code = match code {
        Some(code) => AuthorizationCode::new(code),
        None => return Redirect::to(uri!("http://localhost:3000")),
    };

    // Create a client for handling authentication.
    let client = match auth::make_client() {
        Ok(client) => client,
        Err(_) => return Redirect::to(uri!("http://localhost:3000")),
    };

    // Exchange the code for a token.
    let token_result = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await;

    // Get the access token from the response.
    let access_token: oauth2::AccessToken = match token_result {
        Ok(token) => token.access_token().clone(),
        Err(_) => return Redirect::to(uri!("http://localhost:3000")),
    };

    // Get the GitHub user from the access token.
    let github_user = auth::fetch_user(access_token.clone()).await.unwrap();

    // Sync up info from GitHub with the user and profile tables, returning a session.
    let session = auth::sync_user(&mut conn, github_user).await.unwrap();

    let jwt = encode(
        &Header::default(),
        &session,
        &EncodingKey::from_secret(access_token.secret().as_ref()),
    );

    Redirect::to(format!("http://localhost:3000?token={}", jwt.unwrap()))
}
