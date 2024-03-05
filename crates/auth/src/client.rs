use std::collections::HashMap;

use crate::providers::{GitHubProvider, LocalProvider};
use crate::Config;
use crate::{error::Result, Provider};
use db::Database;

#[derive(Clone)]
pub struct Client {
    provider: Provider,
    github_provider: Option<GitHubProvider>,
    local_provider: Option<LocalProvider>,
}

impl Client {
    /// Creates a new client based on the given config.
    pub fn new(config: Config) -> Self {
        // HACK I wanted to use some trait here that would be opaque to the
        // store crate, but I don't know how to do that.
        let provider = config.provider;
        match provider {
            Provider::GitHub => {
                let github_provider = GitHubProvider::new(
                    config.client_id,
                    config.client_secret,
                    config.callback_url,
                    config.redirect_url,
                );
                Self {
                    provider,
                    github_provider: Some(github_provider),
                    local_provider: None,
                }
            }
            Provider::Local => {
                let local_provider = LocalProvider::new(config.redirect_url);
                Self {
                    provider,
                    github_provider: None,
                    local_provider: Some(local_provider),
                }
            }
        }
    }

    // Performs login based on the provider, returning a URL to redirect the user to.
    pub fn login(self) -> String {
        tracing::info!("GitHub login");
        match self.provider {
            Provider::GitHub => self.github_provider.unwrap().login(),
            Provider::Local => self.local_provider.unwrap().login(),
        }
    }

    /// Logs out the user from the provider.
    pub fn logout(self) {
        match self.provider {
            Provider::GitHub => self.github_provider.unwrap().logout(),
            Provider::Local => self.local_provider.unwrap().logout(),
        }
    }

    /// Handles a callback from the provider.
    pub async fn handle_callback(self, db: Database, params: HashMap<String, String>) -> String {
        match self.provider {
            Provider::GitHub => {
                self.github_provider
                    .unwrap()
                    .handle_callback(db, params)
                    .await
            }
            Provider::Local => {
                self.local_provider
                    .unwrap()
                    .handle_callback(db, params)
                    .await
            }
        }
    }

    pub fn validate_session(self, db: Database, session: &str) -> Result<()> {
        match self.provider {
            Provider::GitHub => self.github_provider.unwrap().validate_session(db, session),
            Provider::Local => self.local_provider.unwrap().validate_session(db, session),
        }
    }
}
