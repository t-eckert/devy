use super::{providers::GitHubProvider, token::Encoder};

#[derive(Clone, Debug)]
pub struct Client {
    pub encoder: Encoder,
    pub github_provider: GitHubProvider,
}

impl Client {
    pub fn new(encoder: Encoder, github_provider: GitHubProvider) -> Self {
        Self {
            encoder,
            github_provider,
        }
    }
}
