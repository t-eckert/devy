use jsonwebtoken::{encode, EncodingKey, Header};
use oauth2::reqwest::async_http_client;
use oauth2::{basic::BasicClient, AuthorizationCode, ClientId, CsrfToken};
use oauth2::{AuthUrl, ClientSecret, TokenResponse, TokenUrl};
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
    let (auth_url, csrf_token) = get_client().authorize_url(CsrfToken::new_random).url();

    Redirect::to(auth_url.clone().to_string())
}

#[get("/logout")]
async fn logout() -> Redirect {
    Redirect::to(uri!("http://localhost:3000"))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[get("/callback?<code>")]
async fn callback(code: Option<String>) -> Redirect {
    if code.is_none() {
        return Redirect::to(uri!("http://localhost:3000"));
    }

    let token_result = get_client()
        .exchange_code(AuthorizationCode::new(code.unwrap()))
        .request_async(async_http_client)
        .await;

    let claims = Claims {
        sub: "1234567890".to_owned(),
        company: "ACME".to_owned(),
        exp: 1_613_952_000,
    };

    let token = token_result.unwrap();
    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(token.access_token().secret().as_ref()),
    );
    println!("Token: {:?}", jwt);

    Redirect::to(format!("http://localhost:3000?token={}", jwt.unwrap()))
}
