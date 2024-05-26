use crate::auth::Client;
use crate::db::Database;
use crate::github::GitHubClient;
use crate::uploader::Uploader;

/// Store is the shared state of the application.
/// It contains a database pool, an authentication client, and an uploader.
#[derive(Clone)]
pub struct Store {
    pub db: Database,
    pub auth_client: Client,
    pub uploader: Uploader,
    pub github_client: GitHubClient,
}

impl Store {
    pub fn new(
        db: Database,
        auth_client: Client,
        uploader: Uploader,
        github_client: GitHubClient,
    ) -> Self {
        Self {
            db,
            auth_client,
            uploader,
            github_client,
        }
    }
}
