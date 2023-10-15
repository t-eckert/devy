use oauth2::reqwest::async_http_client;
use oauth2::TokenResponse;
use oauth2::{
    basic::BasicClient, AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    Scope, TokenUrl,
};
use reqwest;

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

#[derive(Clone)]
pub struct Client {
    oauth_client: BasicClient,
    post_auth_redirect_uri: String,
}

impl Client {
    pub fn new(client_id: String, client_secret: String, post_auth_redirect_uri: String) -> Self {
        Client {
            oauth_client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(AUTH_URL.to_string()).expect("Invalid auth URL"),
                Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token URL")),
            ),
            post_auth_redirect_uri,
        }
    }

    /// Returns the URL to redirect the user to for authorization on GitHub.
    pub fn login_url(&self) -> String {
        let (authorize_url, _) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("repo".to_string()))
            .add_scope(Scope::new("user".to_string()))
            .url();

        authorize_url.to_string()
    }

    // Exchange the code for a token.
    pub async fn exchange_code(&self, code: String) -> Result<AccessToken, anyhow::Error> {
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow::anyhow!("Failed to exchange code: {}", err))?;

        Ok(token.access_token().clone())
    }

    // Returns the URL to redirect the user to after authorization.
    pub fn post_auth_redirect_uri(&self) -> String {
        self.post_auth_redirect_uri.to_string()
    }

    pub async fn fetch_github_user(
        &self,
        token: AccessToken,
    ) -> Result<super::GitHubUser, reqwest::Error> {
        let req_client = reqwest::Client::new();
        let response = req_client
            .get("https://api.github.com/user")
            .header("User-Agent", "rust-rocket-oauth2")
            .header("Accept", "application/json")
            .bearer_auth(token.secret())
            .send()
            .await?;

        let user: super::GitHubUser = response.json().await?;

        Ok(user)
    }
}
