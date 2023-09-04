use crate::auth;
use jsonwebtoken::{encode, EncodingKey, Header};
use oauth2::reqwest::async_http_client;
use oauth2::{basic::BasicClient, AuthorizationCode, ClientId, CsrfToken};
use oauth2::{AuthUrl, ClientSecret, Scope, TokenResponse, TokenUrl};
use rocket::response::Redirect;
use serde::{Deserialize, Serialize};

// TODO move this into the auth mod

const GITHUB_AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

fn get_client_id() -> String {
    dotenvy::var("CLIENT_ID").expect("CLIENT_ID must be set")
}

fn get_client_secret() -> String {
    dotenvy::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set")
}

fn get_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(get_client_id()),
        Some(ClientSecret::new(get_client_secret())),
        AuthUrl::new(GITHUB_AUTH_URL.to_string()).expect("Invalid auth url"),
        Some(TokenUrl::new(GITHUB_TOKEN_URL.to_string()).expect("Invalid token url")),
    )
}

// ^^^

pub fn routes() -> Vec<rocket::Route> {
    routes![login, logout, callback]
}

/// login redirects the user to GitHub's auth page
#[get("/login?<redirect>")]
async fn login(redirect: Option<String>) -> Redirect {
    let auth_url = auth::get_uri().unwrap();
    Redirect::to(auth_url.clone().to_string())
}

#[get("/logout")]
async fn logout() -> Redirect {
    Redirect::to(uri!("http://localhost:3000"))
}

#[get("/callback?<code>")]
async fn callback(code: Option<String>) -> Redirect {
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

    // TODO exchange the github_user for a session. Sync the github user with
    // user info in our database and profile info. Take GitHub as the source of truth.

    let jwt = encode(
        &Header::default(),
        &github_user,
        &EncodingKey::from_secret(access_token.secret().as_ref()),
    );

    Redirect::to(format!("http://localhost:3000?token={}", jwt.unwrap()))
}
