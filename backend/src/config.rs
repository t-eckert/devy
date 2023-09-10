use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    /// The URL to redirect to after authentication.
    pub post_auth_redirect: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        let post_auth_redirect =
            std::env::var("POST_AUTH_REDIRECT").context("POST_AUTH_REDIRECT must be set")?;

        Ok(Config { post_auth_redirect })
    }
}
