/* TODO
 * For now, each login request creates a new instance of the client. This is not very efficient.
 * We should create a single instance of the client and store it in the app state.
 */

use crate::db::DB;
use oauth2::reqwest::async_http_client;
use oauth2::EmptyExtraTokenFields;
use oauth2::{
    basic::BasicClient, basic::BasicTokenType, AccessToken, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, StandardTokenResponse, TokenUrl,
};
use reqwest;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use std::env;

use crate::entities::{Profile, User, UserController};

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

pub fn make_client() -> Result<BasicClient, anyhow::Error> {
    let client_id = ClientId::new(dotenvy::var("CLIENT_ID")?);
    let client_secret = Some(ClientSecret::new(dotenvy::var("CLIENT_SECRET")?));
    let auth_url = AuthUrl::new(AUTH_URL.to_string())?;
    let token_url = Some(TokenUrl::new(TOKEN_URL.to_string())?);

    Ok(BasicClient::new(
        client_id,
        client_secret,
        auth_url,
        token_url,
    ))
}

pub fn get_uri() -> Result<String, anyhow::Error> {
    let client = make_client()?;

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("repo".to_string()))
        .add_scope(Scope::new("user".to_string()))
        .url();

    Ok(authorize_url.to_string())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubUser {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    site_admin: bool,
    name: String,
    company: String,
    blog: String,
    location: String,
    email: String,
    hireable: Option<bool>,
    bio: String,
    twitter_username: String,
    public_repos: i64,
    public_gists: i64,
    followers: i64,
    following: i64,
    created_at: String,
    updated_at: String,
}

pub async fn fetch_user(access_token: AccessToken) -> Result<GitHubUser, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user")
        .header("User-Agent", "rust-rocket-oauth2")
        .header("Accept", "application/json")
        .bearer_auth(access_token.secret())
        .send()
        .await?;

    let user: GitHubUser = response.json().await?;

    Ok(user)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    user: User,
    // profile: Profile,
}

pub async fn sync_user(
    db: Connection<DB>,
    github_user: GitHubUser,
) -> Result<Session, anyhow::Error> {
    // TODO It will be better to do this by looking up the immutable GitHub user ID. I have to add
    // it to the database first.

    let user_controller = UserController::new(db);

    match user_controller
        .get_by_github_username(github_user.login)
        .await
    {
        Some(user) => Ok(Session { user }),
        None => unimplemented!(),
    }
}
