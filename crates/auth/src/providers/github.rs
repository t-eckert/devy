use crate::error::{Error, Result};
use crate::Provider;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, Scope, TokenResponse, TokenUrl,
};

pub struct GitHubProvider {
    oauth_client: BasicClient,
    callback_url: String,
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
