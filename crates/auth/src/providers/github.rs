use crate::error::Result;
use db::Database;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};

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

    pub fn login(&self) {
        println!("Logging in with GitHub");
    }

    pub fn logout(&self) {
        println!("Logging out from GitHub");
    }

    pub fn handle_callback(&self) -> Result<()> {
        println!("Handling GitHub callback");
        Ok(())
    }

    pub fn validate_session(&self, _session: &str, _db: Database) -> Result<()> {
        println!("Validating GitHub session");
        Ok(())
    }
}
