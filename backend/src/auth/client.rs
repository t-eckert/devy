use super::{
    error::{Error, Result},
    GitHubUser,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, Scope, TokenResponse, TokenUrl,
};
use reqwest;

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

/// Client is the Oauth2 client for managing GitHub authentication.
#[derive(Clone)]
pub struct Client {
    pub post_auth_redirect_uri: String,

    oauth_client: BasicClient,
    callback_url: String,
}

impl Client {
    pub fn new(
        client_id: String,
        client_secret: String,
        post_auth_redirect_uri: String,
        callback_url: String,
    ) -> Self {
        Client {
            oauth_client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(AUTH_URL.to_string()).expect("Invalid auth URL"),
                Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token URL")),
            ),
            post_auth_redirect_uri,
            callback_url,
        }
    }

    /// Returns the URL to redirect the user to for authorization on GitHub.
    pub fn login_url(&self) -> String {
        let (authorize_url, _) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("repo".to_string()))
            .add_scope(Scope::new("user".to_string()))
            .add_extra_param("redirect_uri", &self.callback_url)
            .url();

        authorize_url.to_string()
    }

    // Exchange the code for a token.
    pub async fn exchange_code(&self, code: String) -> Result<AccessToken> {
        match self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
        {
            Ok(token) => Ok(token.access_token().clone()),
            Err(err) => Err(Error::CodeExchangeForTokenFailed(err.to_string())),
        }
    }

    // Returns the GitHub user associated with the token.
    pub async fn fetch_github_user(&self, token: AccessToken) -> Result<GitHubUser> {
        match reqwest::Client::new()
            .get("https://api.github.com/user")
            .header("User-Agent", "devy-backend")
            .header("Accept", "application/json")
            .bearer_auth(token.secret())
            .send()
            .await
        {
            Ok(response) => Ok(response
                .json()
                .await
                .map_err(|err| Error::UnableToDeserializeUser(err.to_string()))?),
            Err(err) => Err(Error::TokenExchangeForUserFailed(err.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_login_url() {
        let client = Client::new(
            "Iv1.xf49281766572361".to_string(),
            "9e5287f378e7f9af6041fd484314845115a4c211".to_string(),
            "https://devy.page".to_string(),
            "https://api.devy.page/v1/auth/callback".to_string(),
        );

        let url = client.login_url();
        assert!(url.contains("https://github.com/login/oauth/authorize"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=Iv1.xf49281766572361"));
        assert!(url.contains("scope=repo+user"));
        assert!(url.contains("redirect_uri=https%3A%2F%2Fapi.devy.page%2Fv1%2Fauth%2Fcallback"));
    }
}
