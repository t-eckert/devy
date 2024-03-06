use crate::error::{Error, Result};
use db::Database;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, Scope, TokenResponse, TokenUrl,
};
use std::{collections::HashMap, str::FromStr};

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

#[derive(Clone)]
pub struct Client {
    oauth_client: BasicClient,
    callback_url: String,
    redirect_url: String,
}

impl Client {
    /// Creates a new client based on the given config.
    pub fn new(
        client_id: String,
        client_secret: String,
        callback_url: String,
        redirect_url: String,
    ) -> Self {
        Self {
            oauth_client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(AUTH_URL.to_string()).expect("Invalid auth URL"),
                Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token URL")),
            ),
            callback_url,
            redirect_url,
        }
    }

    // Performs login based on the provider, returning a URL to redirect the user to.
    pub fn login(self) -> String {
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

    /// Logs out the user from the provider.
    pub fn logout(self) {
        unimplemented!()
    }

    // Handles a callback from GitHub by exchanging the code for a token and then
    // exchanging the token for a user.
    // User information is used to upsert a user and profile in the database.
    // A session is created for the user.
    // This session is encoded as a JWT and attached as a query param to the redirect URL.
    // At the end of the process, the redirect URL is returned with the attached query param.
    // If the process fails, the redirect URL is returned without the query param.
    pub async fn handle_callback(self, db: Database, params: HashMap<String, String>) -> String {
        tracing::info!("Handling GitHub callback.");

        // Get the code passed from GitHub in the query params.
        let code = match params.get("code") {
            Some(code) => code,
            None => return self.redirect_url_with_err("Unable to get Code".to_string()),
        };

        let access_token = match self.exchange_code(code).await {
            Ok(token) => token,
            Err(err) => return self.redirect_url_with_err(err.to_string()),
        };

        let github_user = match self.fetch_github_user(access_token.clone()).await {
            Ok(user) => user,
            Err(err) => return self.redirect_url_with_err(err.to_string()),
        };

        let user = match db::user::upsert(
            &db,
            github_user.login.clone().unwrap_or(random_username()),
            github_user.email.clone(),
            github_user.login.clone(),
            Some("user".to_string()),
            Some("active".to_string()),
        )
        .await
        {
            Ok(user) => user,
            Err(err) => return self.redirect_url_with_err(err.to_string()),
        };

        let profile = match db::profile::upsert(
            &db,
            None,
            Some(user.id),
            github_user.name.clone().unwrap_or("unnamed".to_string()),
        )
        .await
        {
            Ok(profile) => profile,
            Err(err) => return self.redirect_url_with_err(err.to_string()),
        };

        let secret = crate::generate_encoding_key();

        let session = entities::Session::new(
            None,
            user.id,
            uuid::Uuid::from_str(&profile.id.unwrap()).unwrap(),
            user.username.clone(),
            profile.display_name.clone(),
            profile.avatar_url.clone(),
            user.role.clone(),
            user.status.clone(),
            "access token".to_string(),
            secret,
        );

        // let session = match db::session::insert(&db, session).await {
        //     Ok(session) => session,
        //     Err(_) => return self.redirect_url.clone(),
        // };
        //

        let jwt = match session.encode() {
            Ok(token) => token,
            Err(err) => return self.redirect_url_with_err(err.to_string()),
        };

        /*


            // Encode the session as a JWT.
            let jwt = match encode(
                &Header::default(),
                &session,
                &EncodingKey::from_secret(secret.as_bytes()),
            ) {
                Ok(token) => token,
                Err(err) => {
                    eprintln!("Failed to encode JWT: {}", err);
                    return Redirect::to(&store.auth_client.post_auth_redirect_uri);
                }
            };

            // Redirect to the post-auth redirect URI with the JWT as a query param.
            Redirect::to(
                format!(
                    "{}?token={}",
                    &store.auth_client.post_auth_redirect_uri, jwt
                )
                .as_str(),
            )
        */

        format!("{}?token={}", self.redirect_url, jwt)
    }

    pub fn validate_session(self, db: Database, session: &str) -> Result<()> {
        unimplemented!();
        Ok(())
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
            Err(err) => Err(Error::CodeExchangeForTokenFailed(err.to_string())),
        }
    }

    // Returns the GitHub user associated with the token.
    async fn fetch_github_user(&self, token: AccessToken) -> Result<GitHubUser> {
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

    fn redirect_url_with_err(self, err: String) -> String {
        format!("{}?error={}", self.redirect_url, err)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub user_type: Option<String>,
    pub site_admin: Option<bool>,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: Option<i64>,
    pub public_gists: Option<i64>,
    pub followers: Option<i64>,
    pub following: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

fn random_username() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen();
    format!("user-{}", n)
}
