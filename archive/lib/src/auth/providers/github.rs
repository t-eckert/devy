use super::{CallbackResponse, Provider};
use crate::{
    auth::{Authentication, Error, Result, Session},
    date::Date,
    db,
    identity::Identity,
    profile::Profile,
    repositories::{IdentityRepository, ProfileRepository, UserRepository},
    user::User,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthUrl, AuthorizationCode,
    ClientId, ClientSecret, CsrfToken, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use url::Url;

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

#[derive(Clone, Debug)]
pub struct GitHubProvider {
    oauth_client: BasicClient,
    callback_url: Url,
}

impl GitHubProvider {
    /// Create a new GitHub provider.
    pub fn new(client_id: String, client_secret: String, callback_url: Url) -> Result<Self> {
        Ok(Self {
            oauth_client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(AUTH_URL.to_string())?,
                Some(TokenUrl::new(TOKEN_URL.to_string())?),
            ),
            callback_url,
        })
    }

    /// Build the URL to redirect the user to for login.
    pub fn login_url(&self) -> Url {
        tracing::info!("Configuring redirect for GitHub login.");
        let (authorize_url, _) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("repo".to_string()))
            .add_scope(Scope::new("user".to_string()))
            .add_extra_param("redirect_uri", &self.callback_url.to_string())
            .url();

        authorize_url
    }

    /// Handle the callback from GitHub by exchanging the code for a token and fetching the user's
    /// profile. This also updates the identity, user, and profile in the database.
    pub async fn handle_callback(
        &self,
        db_conn: &db::Conn,
        code: &str,
    ) -> Result<CallbackResponse> {
        tracing::info!("Handling GitHub callback");

        let access_token = self.exchange_code(code).await?;
        let github_user = self.get_github_user(&access_token).await?;

        let (identity, user, profile) = match get_identity(db_conn, &github_user).await? {
            Some(identity) => (
                identity.clone(),
                update_user(db_conn, &identity).await?,
                update_profile(db_conn, &identity, &github_user).await?,
            ),
            None => create_new_user_from_github_user(db_conn, &github_user).await?,
        };

        Ok(CallbackResponse {
            provider: Provider::GitHub,
            session: Session::new(identity.id),
            authentication: Authentication::new(&identity, &user, &profile),
        })
    }

    async fn exchange_code(&self, code: &str) -> Result<AccessToken> {
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

    async fn get_github_user(&self, token: &AccessToken) -> Result<GitHubUser> {
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

async fn get_identity(db_conn: &db::Conn, github_user: &GitHubUser) -> Result<Option<Identity>> {
    tracing::info!("Attempting to find identity belonging to GitHub user.");

    let github_username = github_user
        .login
        .as_ref()
        .ok_or(Error::GitHubUserHasNoUsername)?;

    // If we can find the identity by the GitHub username, return it.
    match IdentityRepository::get_by_github_username(db_conn, github_username).await? {
        Some(identity) => return Ok(Some(identity)),
        None => (),
    }

    let email = github_user
        .email
        .as_ref()
        .ok_or(Error::GitHubUserHasNoEmail)?;
    let github_user_id = github_user.id.ok_or(Error::GitHubUserHasNoId)?;

    // If we can find the identity by the email, add the GitHub user id to the identity and return it.
    match IdentityRepository::get_by_email(db_conn, email).await? {
        Some(mut identity) => {
            identity.enable_github_auth(github_user_id);
            let id = IdentityRepository::update(db_conn, identity).await?.id;
            return Ok(IdentityRepository::get(db_conn, id).await?);
        }
        None => (),
    }

    Ok(None)
}

async fn create_new_user_from_github_user(
    db_conn: &db::Conn,
    github_user: &GitHubUser,
) -> Result<(Identity, User, Profile)> {
    let user = User::new(
        github_user
            .login
            .as_ref()
            .ok_or(Error::GitHubUserHasNoUsername)?,
    );

    let user = UserRepository::get(db_conn, UserRepository::insert(db_conn, &user).await?.id)
        .await?
        .ok_or(Error::CannotCreateUser)?;

    let profile = Profile::new(user.id, github_user.);

    Ok((identity, user, profile))
}

async fn update_user(db_conn: &db::Conn, identity: &Identity) -> Result<User> {
    let mut user = UserRepository::get(db_conn, identity.user_id)
        .await?
        .ok_or(Error::UserNotFound)?;

    user.last_login_at = Some(Date::now());

    let id = UserRepository::update(db_conn, user.clone()).await?.id;

    Ok(UserRepository::get(db_conn, id)
        .await?
        .ok_or(Error::UserNotFound)?)
}

async fn update_profile(
    db_conn: &db::Conn,
    identity: &Identity,
    github_user: &GitHubUser,
) -> Result<Profile> {
    let mut profile = ProfileRepository::get(db_conn, identity.profile_id)
        .await?
        .ok_or(Error::ProfileNotFound)?;

    if profile.display_name.is_none() {
        profile.display_name = github_user.name.clone();
    }
    if profile.avatar_url.is_none() {
        profile.avatar_url = github_user.avatar_url.clone();
    }
    if profile.bio.is_none() {
        profile.bio = github_user.bio.clone();
    }

    let id = ProfileRepository::update(db_conn, &profile).await?.id;

    Ok(ProfileRepository::get(db_conn, id)
        .await?
        .ok_or(Error::ProfileNotFound)?)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GitHubUser {
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
