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
                );
                Self {
                    provider,
                    github_provider: Some(github_provider),
                    local_provider: None,
                }
            }
            Provider::Local => {
                let local_provider = LocalProvider::new();
                Self {
                    provider,
                    github_provider: None,
                    local_provider: Some(local_provider),
                }
            }
        }
    }

    pub fn login(self) {
        match self.provider {
            Provider::GitHub => self.github_provider.unwrap().login(),
            Provider::Local => self.local_provider.unwrap().login(),
        }
    }

    pub fn logout(self) {
        match self.provider {
            Provider::GitHub => self.github_provider.unwrap().logout(),
            Provider::Local => self.local_provider.unwrap().logout(),
        }
    }

    pub fn handle_callback(self) -> Result<()> {
        match self.provider {
            Provider::GitHub => self.github_provider.unwrap().handle_callback(),
            Provider::Local => self.local_provider.unwrap().handle_callback(),
        }
    }

    pub fn validate_session(self, session: &str, db: Database) -> Result<()> {
        match self.provider {
            Provider::GitHub => self.github_provider.unwrap().validate_session(session, db),
            Provider::Local => self.local_provider.unwrap().validate_session(session, db),
        }
    }
}
