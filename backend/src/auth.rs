use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use std::env;

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

pub fn setup() {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let client_secret = ClientSecret::new(
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
    );

    let auth_url = AuthUrl::new(AUTH_URL.to_string()).unwrap();
    let token_url = TokenUrl::new(TOKEN_URL.to_string()).unwrap();

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url));

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("repo".to_string()))
        .add_scope(Scope::new("user".to_string()))
        .url();
}
