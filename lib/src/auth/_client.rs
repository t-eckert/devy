use super::{
    error::{Error, Result},
    Provider,
};
use crate::{
    auth::token::{Encoder, Session},
    db,
    github::GitHubUser,
    repositories::{ProfileRepository, UserRepository},
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, Scope, TokenResponse, TokenUrl,
};
use std::collections::HashMap;

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

/// A client for managing user authentication.
#[derive(Clone, Debug)]
pub struct Client {
    pub redirect_url: String,
    callback_url: String,
    oauth_client: BasicClient,
    encoder: Encoder,
}

impl Client {
    /// Creates a new client based on the given config.
    pub fn new(
        client_id: String,
        client_secret: String,
        callback_url: String,
        redirect_url: String,
        private_pem: &[u8],
        public_pem: &[u8],
    ) -> Self {
        Self {
            redirect_url,
            callback_url,
            oauth_client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(AUTH_URL.to_string()).expect("Invalid auth URL"),
                Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token URL")),
            ),
            encoder: Encoder::new(public_pem, private_pem).unwrap(),
        }
    }

    pub fn provider_auth_url(&self, provider: Provider) -> String {
        match provider {
            Provider::GitHub => self.login(),
        }
    }

    // Performs login based on the provider, returning a URL to redirect the user to.
    pub fn login(&self) -> String {
        tracing::info!("Configuring redirect for GitHub login.");
        let (authorize_url, _) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("repo".to_string()))
            .add_scope(Scope::new("user".to_string()))
            .add_extra_param("redirect_uri", &self.callback_url)
            .url();

        authorize_url.to_string()
    }

    // Handles a callback from GitHub by exchanging the code for a token and then exchanging the token for a user.
    // User information is used to upsert a user and profile in the database.
    // A session is created for the user.
    // This session is encoded as a JWT and attached as a query param to the redirect URL.
    // At the end of the process, the redirect URL is returned with the attached query param.
    // If the process fails, the redirect URL is returned without the query param.
    pub async fn handle_callback(
        self,
        db_conn: &db::Conn,
        params: HashMap<String, String>,
    ) -> Result<String> {
        tracing::info!("Handling GitHub callback.");

        // Get the code passed from GitHub in the query params.
        let code = params
            .get("code")
            .ok_or(Error::CodeExchangeForTokenFailed)?;

        let access_token = self.exchange_code(code).await?;
        let github_user = self.fetch_github_user(access_token.clone()).await?;

        let github_username = github_user
            .clone()
            .login
            .ok_or(Error::GitHubUserHasNoUsername)?;

        let user = match UserRepository::get_by_username(db_conn, &github_username).await? {
            Some(user) => {
                let id = UserRepository::update(db_conn, user).await?.id;
                UserRepository::get(db_conn, id).await?.unwrap()
            }
            None => {
                let id = UserRepository::insert(
                    db_conn,
                    &github_username,
                    github_user.email.clone().as_deref(),
                    Some(&github_username),
                )
                .await?
                .id;
                UserRepository::get(db_conn, id).await?.unwrap()
            }
        };

        let username = user.username.clone();
        let profile = match ProfileRepository::get_by_username(db_conn, &username).await {
            Ok(mut profile) => {
                profile.update_from_github_user(github_user);
                let id = ProfileRepository::update(db_conn, profile).await?;
                ProfileRepository::get(db_conn, id).await?
            }
            Err(_) => {
                let profile_id = ProfileRepository::insert(
                    db_conn,
                    user.id,
                    Some(github_user.name.clone().unwrap_or("unnamed".to_string())),
                    github_user.avatar_url,
                    None,
                    None,
                )
                .await?;
                ProfileRepository::get(db_conn, profile_id).await?
            }
        };

        let session = Session::new(
            user.id,
            profile.id,
            user.username,
            user.role,
            user.status,
            profile.display_name,
            profile.avatar_url,
        );

        let token = self.encoder.encode(session)?;

        Ok(token)
    }

    pub fn redirect_url_with_token(self, token: &str) -> String {
        format!("{}?token={}", self.redirect_url, token)
    }

    pub fn redirect_url_with_err(self, err: &str) -> String {
        format!("{}?error={}", self.redirect_url, err)
    }

    // Validates a token and returns the associated session.
    pub async fn validate_token(self, token: &str) -> Result<Session> {
        Ok(self.encoder.decode(token)?)
    }

    // Exchange the code for a token.
    async fn exchange_code(&self, code: &String) -> Result<AccessToken> {
        match self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await
        {
            Ok(token) => Ok(token.access_token().clone()),
            Err(_) => Err(Error::CodeExchangeForTokenFailed),
        }
    }

    // Returns the GitHub user associated with the token.
    async fn fetch_github_user(&self, token: AccessToken) -> Result<GitHubUser> {
        match reqwest::Client::new()
            .get("https://api.github.com/user")
            .header("User-Agent", "devy")
            .header("Accept", "application/json")
            .bearer_auth(token.secret())
            .send()
            .await
        {
            Ok(response) => Ok(response
                .json()
                .await
                .map_err(|err| Error::UnableToDeserializeUser(err.to_string()))?),
            Err(err) => Err(err.into()),
        }
    }
}
