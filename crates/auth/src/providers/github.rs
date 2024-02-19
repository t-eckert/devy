use super::Provider;
use crate::error::Result;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, Scope, TokenResponse, TokenUrl,
};

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

#[derive(Clone)]
pub struct GitHubProvider {
    oauth_client: BasicClient,
    callback_url: String,
}

impl GitHubProvider {
    pub fn new(client_id: String, client_secret: String, callback_url: String) -> Self {
        Self {
            oauth_client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(AUTH_URL.to_string()).expect("Invalid auth URL"),
                Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token URL")),
            ),
            callback_url,
        }
    }
}

impl Provider for GitHubProvider {
    fn login(&self) {
        println!("Logging in with GitHub");
    }

    fn logout(&self) {
        println!("Logging out from GitHub");
    }

    fn handle_callback(&self) -> Result<()> {
        Ok(())
    }
}
