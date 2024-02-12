mod client;
mod error;
mod github_user;
mod session;
mod validate;

pub use client::Client;
pub use github_user::GitHubUser;
pub use session::generate_encoding_key;
pub use validate::is_authenticated;

use std::env;

/// Reads the environment variables and creates a new Client.
/// This will panic if any of the environment variables are not set.
/// CLIENT_ID - The GitHub OAuth2 client ID.
/// CLIENT_SECRET - The GitHub OAuth2 client secret.
/// POST_AUTH_URI - The URI to redirect to after authentication.
/// CALLBACK_URL - The URL GitHub will redirect to for issuing the callback.
pub fn setup_client() -> Client {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let post_auth_redirect_uri = env::var("POST_AUTH_URI").expect("POST_AUTH_URI not set");
    let callback_url = env::var("CALLBACK_URL").expect("CALLBACK_URL not set");

    Client::new(
        client_id,
        client_secret,
        post_auth_redirect_uri,
        callback_url,
    )
}
